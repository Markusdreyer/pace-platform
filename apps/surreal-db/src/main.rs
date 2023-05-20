mod models;
// use crate::models::Image;
use crate::models::Name;
use crate::models::User;

use chrono::Utc;
use serde::Deserialize;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    println!("main()");

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

    let name = Name::new("Tobias".to_string(), Some("Schultz".to_string()));
    let mut user_new = User::new();
    let user = user_new.set_name(name).set_last_online(Utc::now());

    // Create a new person with a random id
    let created: Record = db.create("user").content(user).await?;
    dbg!(created);

    // // Update a person record with a specific id
    // let updated: Record = db
    //     .update(("person", "jaime"))
    //     .merge(Responsibility { marketing: true })
    //     .await?;
    // dbg!(updated);

    // Select all people records
    let users: Vec<Record> = db.select("users").await?;
    dbg!(users);

    // Perform a custom advanced query
    // let groups = db
    //     .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
    //     .bind(("table", "person"))
    //     .await?;
    // dbg!(groups);

    Ok(())
}
