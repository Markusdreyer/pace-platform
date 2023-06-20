use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::task;
use tokio::time::sleep;
use tracing::{error, info};
use tungstenite::{connect, Message};
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
    let mut update_iter = location_updates.iter();
    let mut rng = rand::rngs::StdRng::from_entropy();

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:8080/race/race1").unwrap()).expect("Can't connect");

    loop {
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let location_update = match update_iter.next() {
            Some(update) => LocationUpdate {
                user_id: update.user_id.clone(),
                timestamp,
                coordinates: Coordinates {
                    lat: update.coordinates.lat,
                    long: update.coordinates.long,
                },
            },
            None => return Ok(()), // Stop the loop if there are no more updates
        };

        // Add randomness to the coordinates

        let location_update = LocationUpdate {
            user_id: client_id.clone(),
            timestamp,
            coordinates: Coordinates {
                lat: location_update.coordinates.lat,
                long: location_update.coordinates.long,
            },
        };

        let json = serde_json::to_string(&location_update).unwrap();

        socket.write_message(Message::Text(json)).unwrap();

        sleep(Duration::from_millis(1000)).await; // pause before sending the next location update
    }
}
