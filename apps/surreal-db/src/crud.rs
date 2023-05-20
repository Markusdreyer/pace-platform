mod db_client;

#[derive(Debug)]
pub fn getUser(id: String) -> Result<User, Error> {
    let db = db::Database::new().await?.db;
    let collection = db.collection("users");
}
