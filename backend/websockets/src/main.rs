use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::web::{Data, Path, Payload};
use actix_web::{get, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actors::race::Race;
use prometheus::Encoder;
use shared::log::configure_log;
use shared::{model::Settings, setup_config};
use tracing::info;
use uuid::Uuid;

use crate::actors::ws::WsConnection;
use crate::prom::{register_custom_metrics, INCOMING_REQUESTS, REGISTRY};

mod actors;
mod prom;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Settings = setup_config().expect("could not setup config");
    configure_log(config.log.level);
    register_custom_metrics();

    let race_server = Race::default().start();

    info!("Starting server");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(metrics)
            .service(establish_connection)
            .app_data(Data::new(race_server.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/race/{race_id}")]
pub async fn establish_connection(
    req: HttpRequest,
    stream: Payload,
    race_id: Path<String>,
    srv: Data<Addr<Race>>,
) -> Result<HttpResponse, Error> {
    info!(message = "new connection", action = "establish_connection");

    INCOMING_REQUESTS.inc();

    let user_id = Uuid::new_v4().to_string();
    let ws = WsConnection::new(user_id, race_id.to_string(), srv.get_ref().clone());
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

#[get("/metrics")]
pub async fn metrics() -> Result<HttpResponse, Error> {
    // Gather the metrics
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        print!("Error encoding metrics: {e}")
    }

    let metrics = String::from_utf8(buffer).unwrap();

    Ok(HttpResponse::Ok().body(metrics))
}
