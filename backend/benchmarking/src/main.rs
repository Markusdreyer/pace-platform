use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use shared::log::configure_log;
use shared::model::Settings;
use shared::setup_config;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage};
use tracing::{debug, error, info};
use tungstenite::Message;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LocationUpdate {
    user_id: String,
    timestamp: u64,
    coordinates: Coordinates,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coordinates {
    lat: f64,
    long: f64,
}

const TOTAL_CLIENTS: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Settings = setup_config().expect("could not setup config");
    configure_log(config.log.level);
    let mut handles = vec![];

    for i in 0..TOTAL_CLIENTS {
        let client_id = format!("client{i}");

        let handle = task::spawn(simulate_client(client_id));
        handles.push(handle);
        sleep(Duration::from_millis(1000)).await; // pause before spawning next client
    }

    for handle in handles {
        let res = handle.await?;
        info!("Client finished: {:?}", res)
    }
    Ok(())
}

async fn simulate_client(client_id: String) -> Result<(), Box<dyn Error + Send>> {
    let file = File::open("location_updates.json").expect("could not open file");
    let reader = BufReader::new(file);
    let location_updates: Vec<LocationUpdate> = from_reader(reader).expect("could not parse json");

    let (ws_stream, _) = connect_async(Url::parse("ws://localhost:8080/race/race1").unwrap())
        .await
        .expect("Can't connect");

    let (mut write, mut read) = ws_stream.split();

    // Create a channel for sending messages to be written
    let (write_tx, mut write_rx): (UnboundedSender<TMessage>, UnboundedReceiver<TMessage>) =
        mpsc::unbounded_channel();

    let mut update_interval = tokio::time::interval(Duration::from_millis(1000));

    let mut update_iter = location_updates.iter();

    let writer = async {
        while let Some(update) = update_iter.next() {
            update_interval.tick().await;
            let now = SystemTime::now();
            let timestamp = now
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            let location_update = LocationUpdate {
                user_id: client_id.clone(),
                timestamp,
                coordinates: Coordinates {
                    lat: update.coordinates.lat,
                    long: update.coordinates.long,
                },
            };

            let json = serde_json::to_string(&location_update).unwrap();
            write
                .send(TMessage::Text(json))
                .await
                .expect("could not send message");
        }

        write_tx.send(TMessage::Close(None)).unwrap(); // send close message to reader

        while let Some(message) = write_rx.recv().await {
            match message {
                TMessage::Close(_) => break,
                _ => write
                    .send(message)
                    .await
                    .map_err(|e| -> Box<dyn Error + Send> { Box::new(e) })?,
            }
        }

        info!("Client finished");
        Ok::<(), Box<dyn Error + Send>>(())
    };

    let reader = async {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Ping(ping)) => {
                    debug!(message = "Received ping", user_id = client_id);
                    write_tx.send(Message::Pong(ping)).unwrap();
                }
                Ok(Message::Close(_)) => break, // Break the loop when close message is received
                Ok(_) => {}
                Err(e) => {
                    error!("Error reading message: {:?}", e);
                }
            }
        }
        Ok::<(), Box<dyn Error + Send>>(())
    };

    tokio::select! {
        res = writer => res?,
        res = reader => res?,
    };

    Ok(())
}
