use actix::{
    fut, Actor, ActorContext, ActorFuture, Addr, ContextFutureSpawner, Handler, Running,
    StreamHandler, WrapFuture,
};
use actix::{AsyncContext, Message};
use actix_web_actors::ws;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use shared::model::Topics;
use std::time::{Duration, Instant};
use tracing::{debug, error};

use super::messages::{Connect, Disconnect, LocationUpdateMessage, WsMessage};
use super::race::Race;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsConnection {
    pub user_id: String,
    pub race_id: String,
    pub race_addr: Addr<Race>,
    pub heartbeat: Instant,
    pub kafka_producer: FutureProducer,
    pub kafka_topics: Topics,
}

impl WsConnection {
    pub fn new(
        user_id: String,
        race_id: String,
        race: Addr<Race>,
        kafka_producer: FutureProducer,
        kafka_topics: Topics,
    ) -> WsConnection {
        WsConnection {
            user_id,
            race_id,
            heartbeat: Instant::now(),
            race_addr: race,
            kafka_producer,
            kafka_topics,
        }
    }

    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<WsConnection>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                error!(
                    message = "client heartbeat failed, disconnecting",
                    action = "heartbeat",
                    user_id = ?act.user_id
                );
                act.race_addr.do_send(Disconnect {
                    user_id: act.user_id.clone(),
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"PING");
        });
    }
}

impl Handler<WsMessage> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg.0).unwrap())
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let addr = ctx.address();
        self.race_addr
            .send(Connect {
                addr: addr.recipient(),
                race_id: self.race_id.clone(),
                user_id: self.user_id.clone(),
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.race_addr.do_send(Disconnect {
            user_id: self.user_id.clone(),
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                debug!(message = "ping received", action = "handle", ?msg);
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                debug!(message = "pong received", action = "handle");
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => {
                //These messages comes from the app
                debug!(message = "binary message", action = "handle", ?bin);
                let location_update_message: LocationUpdateMessage = match bin.try_into() {
                    Ok(message) => message,
                    Err(e) => return ctx.text(e.to_json().to_string()),
                };

                let kafka_message = serde_json::to_string(&location_update_message).unwrap();
                let user_id = location_update_message.user_id;

                let kafka_producer = self.kafka_producer.clone();
                let kafka_topic = self.kafka_topics.location_update.clone();
                actix_web::rt::spawn(async move {
                    let result = kafka_producer
                        .send(
                            FutureRecord::to(&kafka_topic)
                                .payload(&kafka_message)
                                .key(&user_id),
                            Timeout::Never,
                        )
                        .await;

                    match result {
                        Ok(_) => {
                            debug!(message = "message sent", action = "handle");
                        }
                        Err(e) => {
                            error!(message = "message not sent", action = "handle", ?e);
                        }
                    }
                });

                //append_msg_to_file(&location_update_message); //This is used to create benchmarking data
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(text)) => {
                //These messages comes from other clients
                debug!(message = "text message", action = "handle", ?text);
                let location_update_message: LocationUpdateMessage = match text.try_into() {
                    Ok(message) => message,
                    Err(e) => return ctx.text(e.to_json().to_string()),
                };

                let kafka_message = serde_json::to_string(&location_update_message).unwrap();
                let user_id = location_update_message.user_id;

                let kafka_producer = self.kafka_producer.clone();
                let kafka_topic = self.kafka_topics.location_update.clone();
                actix_web::rt::spawn(async move {
                    let result = kafka_producer
                        .send(
                            FutureRecord::to(&kafka_topic)
                                .payload(&kafka_message)
                                .key(&user_id),
                            Timeout::Never,
                        )
                        .await;

                    match result {
                        Ok(_) => {
                            debug!(message = "message sent", action = "handle");
                        }
                        Err(e) => {
                            error!(message = "message not sent", action = "handle", ?e);
                        }
                    }
                });
            }
            Err(e) => panic!("{}", e),
        }
    }
}

fn append_msg_to_file(msg: &LocationUpdateMessage) {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::PathBuf;

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("location_updates.json");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    let json = serde_json::to_string(msg).unwrap();
    writeln!(file, "{json},").unwrap();
}
