///
/// It's like a table name in a relational database, just that it's a key to a resource in a surreal database. Same same
///
/// Use it like
/// ```rs
/// surrealdb_client.create(DbResource::User.key()).content(User { ... });
/// ```
#[allow(dead_code)]
pub enum DbResource {
    User,
    Event,
}

impl DbResource {
    pub fn key(&self) -> &str {
        match self {
            DbResource::User => "user",
            DbResource::Event => "event",
        }
    }
}
