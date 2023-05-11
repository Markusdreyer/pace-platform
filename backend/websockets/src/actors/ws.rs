use actix::{fut, Actor, ActorContext, Addr, Running, StreamHandler};
use actix::{AsyncContext, Message};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::messages::{ClientActorMessage, Disconnect};
use super::race::Race;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsConnection {
    pub user_id: Uuid,
    pub race_id: Uuid,
    pub race_addr: Addr<Race>,
    pub heartbeat: Instant,
}

impl WsConnection {
    pub fn new(user_id: Uuid, race_id: Uuid, race: Addr<Race>) -> WsConnection {
        WsConnection {
            user_id,
            race_id,
            heartbeat: Instant::now(),
            race_addr: race,
        }
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat;

        let addr = ctx.address();
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.race_addr.do_send(Disconnect {
            user_id: self.user_id,
            race_id: self.race_id,
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
                self.race_addr.do_send(message);
            }

            Err(e) => panic!("{}", e),
        }
    }
}
