use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::web::{Data, Payload};
use actix_web::{get, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use libs::log::configure_log;
use libs::setup_config;
use tracing::info;
use uuid::Uuid;

use crate::actors::race::Race;
use crate::actors::ws::WsConnection;

mod actors;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: model::Settings = setup_config().expect("could not setup config");
    configure_log(config.log.level);

    let race_server = Race::default().start();

    info!("Starting server");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(establish_connection)
            .app_data(race_server.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/{race_id}")]
pub async fn establish_connection(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Addr<Race>>,
) -> Result<HttpResponse, Error> {
    info!(message = "new connection", action = "establish_connection");
    let race_id: String = req.match_info().get("race_id").unwrap().parse().unwrap();

    let user_id = Uuid::new_v4().to_string();
    let ws = WsConnection::new(user_id, race_id.clone(), srv.get_ref().clone());
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}