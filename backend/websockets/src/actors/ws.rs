use actix::AsyncContext;
use actix::{fut, Actor, Addr, Running};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use uuid::Uuid;

use super::messages::{Connect, Disconnect};
use super::race::Race;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConnection {
    pub user_id: Uuid,
    pub race_id: Uuid,
    pub race_addr: Addr<Race>,
    pub heartbeat: Instant,
}

impl WsConnection {
    fn new(user_id: Uuid, race_id: Uuid, race: Addr<Race>) -> WsConnection {
        WsConnection {
            user_id: Uuid::new_v4(),
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
        self.race_addr
            .send(Connect {
                addr: addr.recipient(),
                race_id: Uuid::new_v4(),
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.race_addr.do_send(Disconnect {
            user_id: self.user_id,
            race_id: self.race_id,
        });
        Running::Stop
    }
}
