use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::web::{Data, Path, Payload};
use actix_web::{get, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actors::race::Race;
use prometheus::Encoder;
use rdkafka::producer::FutureProducer;
use rdkafka::{producer, ClientConfig};
use shared::log::configure_log;
use shared::model::Topics;
use shared::{model::Settings, setup_config};
use tracing::info;
use uuid::Uuid;

use crate::actors::kafka_consumer::{KafkaConsumer, KafkaConsumerActor};
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

    let mut kafka_config = ClientConfig::new();
    kafka_config
        .set("bootstrap.servers", config.kafka.host.as_str())
        .set("group.id", "foo")
        .set("enable.auto.commit", config.kafka.auto_commit.to_string())
        .set("security.protocol", config.kafka.security_protocol.as_str());

    let kafka_producer: producer::FutureProducer = kafka_config
        .create()
        .expect("could not create kafka producer");

    let kafka_consumer = KafkaConsumer::new(
        kafka_config,
        vec![config.kafka.topics.location_update.as_str()],
    )
    .expect("could not create kafka consumer");

    let kafka_consumer_actor = KafkaConsumerActor::new(kafka_consumer, race_server.clone());
    kafka_consumer_actor.start();

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(metrics)
            .service(establish_connection)
            .app_data(Data::new(kafka_producer.clone()))
            .app_data(Data::new(config.kafka.topics.clone()))
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
    kafka_producer: Data<FutureProducer>,
    kafka_topics: Data<Topics>,
) -> Result<HttpResponse, Error> {
    info!(message = "new connection", action = "establish_connection");

    INCOMING_REQUESTS.inc();

    let user_id = Uuid::new_v4().to_string();
    let ws = WsConnection::new(
        user_id,
        race_id.to_string(),
        srv.get_ref().clone(),
        kafka_producer.get_ref().clone(),
        kafka_topics.get_ref().clone(),
    );
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
