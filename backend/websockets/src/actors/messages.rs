use actix::{Message, Recipient};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use shared::WebSocketError;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub LocationUpdateMessage);

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
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Message, Debug, Clone)]
#[rtype(result = "()")]
#[serde(rename_all = "camelCase")]
pub struct LocationUpdateMessage {
    pub user_id: String,
    pub timestamp: u64,
    pub coordinates: Coordinates,
}

impl TryFrom<ws::Message> for LocationUpdateMessage {
    type Error = WebSocketError;

    fn try_from(value: ws::Message) -> Result<Self, WebSocketError> {
        match value {
            ws::Message::Text(text) => {
                let message: LocationUpdateMessage = serde_json::from_str(&text)?;
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
