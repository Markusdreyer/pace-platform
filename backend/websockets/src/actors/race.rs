use std::collections::HashMap;

use actix::{Actor, Context, Handler, Recipient};

use crate::prom::{
    CONNECTED_CLIENTS, DISCONNECTED_CLIENTS, LOCATION_UPDATES_RECEIVED, LOCATION_UPDATES_SENT,
};

use super::messages::{Connect, Disconnect, LocationUpdateMessage, WsMessage};
use tracing::{error, info};

type Socket = Recipient<WsMessage>;

#[derive(Default, Debug)]
pub struct Race {
    participants: HashMap<String, Socket>,
}

impl Race {
    fn send_message(&self, msg: LocationUpdateMessage) {
        for (id, socket) in &self.participants {
            if id != &msg.user_id {
                info!(message = "sending message", action = "send_message", ?msg);
                LOCATION_UPDATES_SENT.inc();
                let _ = socket.do_send(WsMessage(msg.clone()));
            }
        }
    }
}

impl Actor for Race {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Race {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        info!(message = "disconnecting", action = "disconnect_handler", participants = ?self.participants, ?msg);
        CONNECTED_CLIENTS.dec();
        DISCONNECTED_CLIENTS.inc();
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

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        info!(message = "new connection", action = "connect_handler", ?msg);
        CONNECTED_CLIENTS.inc();
        self.participants.insert(msg.user_id, msg.addr);
    }
}

impl Handler<LocationUpdateMessage> for Race {
    type Result = ();

    fn handle(&mut self, msg: LocationUpdateMessage, _ctx: &mut Self::Context) -> Self::Result {
        info!(message = "new message", action = "message_handler");
        LOCATION_UPDATES_RECEIVED.inc();
        self.send_message(msg);
    }
}
