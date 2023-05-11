use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;

use super::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use tracing::error;

type Socket = Recipient<WsMessage>;

#[derive(Default)]
pub struct Race {
    participants: HashMap<Uuid, Socket>,
}

impl Race {
    fn send_message(&self, message: &str) {
        if let Some(socket) = self.participants.get(&Uuid::new_v4()) {
            let _ = socket.do_send(WsMessage(message.to_string()));
        } else {
            error!("Could not find socket")
        }
    }
}

impl Actor for Race {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Race {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        if self.participants.remove(&msg.user_id).is_none() {
            error!("Could not find socket")
        }
    }
}

impl Handler<Connect> for Race {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        self.participants.insert(msg.race_id, msg.addr);
    }
}

impl Handler<ClientActorMessage> for Race {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, ctx: &mut Self::Context) -> Self::Result {
        self.send_message(&msg.user_id.to_string());
    }
}
