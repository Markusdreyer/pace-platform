use rand::{Rng, SeedableRng};
use serde::Serialize;
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::task;
use tokio::time::sleep;
use tungstenite::{connect, Message};
use url::Url;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LocationUpdate {
    user_id: String,
    timestamp: u64,
    coordinates: Coordinates,
}

#[derive(Serialize)]
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
        sleep(Duration::from_secs(1)).await; // pause before spawning next client
    }
    for handle in handles {
        handle.await?;
    }
    Ok(())
}

async fn simulate_client(client_id: String) -> Result<(), Box<dyn Error + Send>> {
    let mut rng = rand::rngs::StdRng::from_entropy();

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:8080/stream").unwrap()).expect("Can't connect");

    loop {
        let now = SystemTime::now();
        let timestamp = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let lat = 37.32475582 + rng.gen::<f64>() * 0.01; // adjust this as needed
        let long = -122.02238087 + rng.gen::<f64>() * 0.01; // adjust this as needed

        let location_update = LocationUpdate {
            user_id: client_id.clone(),
            timestamp,
            coordinates: Coordinates { lat, long },
        };

        let json = serde_json::to_string(&location_update).unwrap();

        socket.write_message(Message::Text(json)).unwrap();

        sleep(Duration::from_secs(1)).await; // pause before sending the next location update

        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {msg}");
    }
}
