use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize)]
pub struct DbRecord {
    id: Thing,
}

pub struct DbClient {
    pub db: Surreal<Client>,
}

impl DbClient {
    pub async fn new() -> Result<Self, surrealdb::Error> {
        let ws_url_dev = "ws://localhost:8080".to_string();
        let ws_url_prod = "ws://localhost:8080".to_string();

        // Connect to the server
        let db = Surreal::new::<Ws>(ws_url_dev).await?;

        // Signin as a namespace, database, or root user
        db.signin(Root {
            username: "pace",
            password: "pace00FACE55",
        })
        .await?;

        // Select a specific namespace / database
        db.use_ns("test").use_db("test").await?;

        Ok(DbClient { db })
    }
}
