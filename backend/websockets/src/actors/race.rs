use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;

use super::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use tracing::{error, info};

type Socket = Recipient<WsMessage>;

#[derive(Default)]
pub struct Race {
    participants: HashMap<Uuid, Socket>,
}

impl Race {
    fn send_message(&self, msg: ClientActorMessage) {
        if let Some(socket) = self.participants.get(&msg.user_id) {
            info!(message = "sending message", action = "send_message", participants = ?self.participants);
            let _ = socket.do_send(WsMessage(msg));
        } else {
            error!(message = "could not find socket", action = "send_message")
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
            error!(
                message = "could not find socket",
                action = "disconnect_handler"
            )
        }
    }
}

impl Handler<Connect> for Race {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        info!(message = "new connection", action = "connect_handler");
        self.participants.insert(msg.race_id, msg.addr);
    }
}

impl Handler<ClientActorMessage> for Race {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, ctx: &mut Self::Context) -> Self::Result {
        info!(message = "new message", action = "message_handler");
        self.send_message(msg);
    }
}
