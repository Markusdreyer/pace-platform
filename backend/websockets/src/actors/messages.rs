use actix::{Message, Recipient};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use shared::WebSocketError;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub ClientActorMessage);

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub race_id: String,
    pub user_id: String,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub race_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub user_id: String,
    pub race_id: String,
    pub coordinates: Coordinates,
}

impl TryFrom<ws::Message> for ClientActorMessage {
    type Error = WebSocketError;

    fn try_from(value: ws::Message) -> Result<Self, WebSocketError> {
        match value {
            ws::Message::Text(text) => {
                let message: ClientActorMessage = serde_json::from_str(&text)?;
                Ok(message)
            }
            _ => Err(WebSocketError::InvalidMessage(
                "message was not text".into(),
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}
