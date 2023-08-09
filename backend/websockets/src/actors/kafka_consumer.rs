use async_stream::stream;
use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, AsyncContext, Context, Handler, StreamHandler, WrapFuture};
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    error::{KafkaError, KafkaResult},
    ClientConfig, Message,
};
use tracing::{error, info};

use super::{messages::LocationUpdateMessage, race::Race};

#[derive(Debug)]
enum KafkaConsumerError {
    KafkaError(KafkaError),
    SerdeError(serde_json::Error),
}

pub struct KafkaConsumer {
    stream_consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(config: ClientConfig, topics: Vec<&str>) -> Result<Self, KafkaError> {
        let stream_consumer: StreamConsumer = config.create()?;
        stream_consumer.subscribe(&topics)?;

        Ok(KafkaConsumer { stream_consumer })
    }

    async fn consume(&mut self) -> Result<LocationUpdateMessage, KafkaConsumerError> {
        match self.stream_consumer.recv().await {
            KafkaResult::Err(err) => {
                error!("could not consume kafka message: {}", err);
                Err(KafkaConsumerError::KafkaError(err))
            }
            KafkaResult::Ok(msg) => match serde_json::from_slice(msg.payload().unwrap()) {
                Ok(location_update) => Ok(location_update),
                Err(e) => {
                    error!("failed to deserialize kafka message: {}", e);
                    Err(KafkaConsumerError::SerdeError(e))
                }
            },
        }
    }
}

pub struct KafkaConsumerActor {
    kafka_consumer: Arc<Mutex<KafkaConsumer>>,
    race_addr: Addr<Race>,
}

impl KafkaConsumerActor {
    pub fn new(kafka_consumer: KafkaConsumer, race_addr: Addr<Race>) -> Self {
        Self {
            kafka_consumer: Arc::new(Mutex::new(kafka_consumer)),
            race_addr,
        }
    }
}

impl Actor for KafkaConsumerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let consumer_clone = self.kafka_consumer.clone();
        let race_addr = self.race_addr.clone();

        // Create a separate async task that runs the loop
        ctx.spawn(
            async move {
                let mut kafka_consumer = match consumer_clone.lock() {
                    Ok(consumer) => consumer,
                    Err(_) => {
                        error!("Failed to acquire lock on Kafka consumer");
                        return;
                    }
                };

                loop {
                    match kafka_consumer.consume().await {
                        Ok(location_update) => {
                            // Directly sending the location_update to race_addr
                            race_addr.do_send(location_update);
                        }
                        Err(e) => error!("error consuming kafka message: {:?}", e),
                    }
                }
            }
            .into_actor(self),
        );
    }
}
