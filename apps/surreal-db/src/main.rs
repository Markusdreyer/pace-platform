mod db_client;
mod models;

use crate::db_client::{DbClient, DbRecord};

use chrono::Utc;
use models::{Name, User};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    println!("main()");
    let client = DbClient::new().await?.db;

    let name = Name::new("Tobias".to_string(), Some("Schultz".to_string()));
    let mut user_new = User::new();
    let user = user_new.set_name(name).set_last_online(Utc::now());

    // Create a new person with a random id
    let created: DbRecord = client.create("user").content(user).await?;
    dbg!(created);

    // // Update a person dbRecord with a specific id
    // let updated: DbRecord = db
    //     .update(("person", "jaime"))
    //     .merge(Responsibility { marketing: true })
    //     .await?;
    // dbg!(updated);

    // Select all people dbRecords
    let users: Vec<DbRecord> = client.select("users").await?;
    dbg!(users);

    // Perform a custom advanced query
    // let groups = db
    //     .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
    //     .bind(("table", "person"))
    //     .await?;
    // dbg!(groups);

    Ok(())
}
