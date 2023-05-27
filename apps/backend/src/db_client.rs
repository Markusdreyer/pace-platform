use axum::extract::FromRef;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DbRecord {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(FromRef, Debug, Clone)]
pub struct Db {
    pub client: Surreal<Client>,
}

impl Db {
    pub async fn new() -> Result<Self, surrealdb::Error> {
        let ws_url_dev = "127.0.0.1:8000";
        let _ws_url_prod = "ws://localhost:8000";
        println!("Connecting to {}", ws_url_dev);

        // Connect to the server
        let client = Surreal::new::<Ws>(ws_url_dev)
            .await
            .map_err(|err| surrealdb::Error::from(err))?;

        println!("Connected to {}", ws_url_dev);

        // Signin as a namespace, database, or root user
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        println!("Signed in as root");
        // Select a specific namespace / database
        client.use_ns("test").use_db("test").await?;

        Ok(Db { client })
    }
}
