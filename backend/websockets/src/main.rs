use crate::actors::messages::ClientActorMessage;
use crate::actors::ws::WsConnection;
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use shared::log::configure_log;
use shared::setup_config;
use tracing::info;

mod actors;
mod model;

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => info!(message = "received ping", ?msg),
            Ok(ws::Message::Text(text)) => {
                info!(message = "received text", ?text);
                self.race_addr.do_send(message)
            }
            Ok(ws::Message::Binary(bin)) => info!(message = "received ping", ?bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    info!("Starting websocket");
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: model::Settings = setup_config().expect("could not setup config");
    configure_log(config.log.level);

    HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
