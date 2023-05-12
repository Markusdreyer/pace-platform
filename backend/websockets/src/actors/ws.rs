use actix::{
    fut, Actor, ActorContext, ActorFuture, Addr, ContextFutureSpawner, Handler, Running,
    StreamHandler, WrapFuture,
};
use actix::{AsyncContext, Message};
use actix_web::guard::Connect;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use tracing::info;
use uuid::Uuid;

use super::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
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
}

impl WsConnection {
    pub fn new(user_id: String, race_id: String, race: Addr<Race>) -> WsConnection {
        WsConnection {
            user_id,
            race_id,
            heartbeat: Instant::now(),
            race_addr: race,
        }
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
        //self.heartbeat(ctx) TODO: Fix this

        let addr = ctx.address();
        self.race_addr
            .send(Connect {
                addr: addr.recipient(),
                race_id: self.race_id.clone(),
                user_id: self.user_id.clone(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
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
            race_id: self.race_id.clone(),
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(s)) => {
                let message: ClientActorMessage = serde_json::from_str::<ClientActorMessage>(&s)
                    .expect("could not parse message"); //TODO: Do proper error handling
                info!(
                    message = "sending location update",
                    action = "stream_handler",
                    addr = ?self.race_addr
                );
                self.race_addr.do_send(message);
            }

            Err(e) => panic!("{}", e),
        }
    }
}
