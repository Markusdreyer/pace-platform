use std::collections::{HashMap, HashSet};

use actix::{Actor, Context, Handler, Recipient};
use uuid::Uuid;

use super::messages::{Connect, Disconnect, WsMessage};
use tracing::error;

type Socket = Recipient<WsMessage>;

pub struct Race {
    participants: HashMap<Uuid, Socket>,
}

impl Default for Race {
    fn default() -> Self {
        Race {
            participants: HashMap::new(),
        }
    }
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
        //Create a new race if necessary and then add the participant
        self.participants.insert(msg.race_id, msg.addr);
    }
}
