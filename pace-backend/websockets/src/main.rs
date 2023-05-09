use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use shared::log::configure_log;
use shared::setup_config;
use tracing::info;

mod model;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => info!(message = "received ping", ?msg),
            Ok(ws::Message::Text(text)) => info!(message = "received text", ?text),
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
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
