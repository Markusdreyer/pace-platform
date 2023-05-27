mod db_client;
mod models;

use axum::{
    extract::{Query, State},
    http::{StatusCode},
    response::{Result},
    routing::{get, post},
    Json, Router,
};
use db_client::{Db, DbRecord};
use models::{user::User, Name};
use serde::Deserialize;
use std::{net::SocketAddr};
use surrealdb::{engine::remote::ws::Client, Surreal};





mod utils;
use crate::models::DbResource;

// #[derive(OpenApi)]
// struct ApiDoc;

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("main()");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}

async fn app() -> Router {
    println!("app()");
    let db = match Db::new().await {
        Ok(db) => db,
        Err(err) => {
            panic!("Error: {}", err);
        }
    };

    println!("router init()");
    Router::new()
        .route("/", get(root))
        .route("/user/list", get(get_users))
        .route("/user/create", post(create_user))
        .with_state(db.client)
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    println!("root()");
    "works 💪"
}

#[axum_macros::debug_handler]
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(db): State<Surreal<Client>>,
    Json(payload): Json<User>,
) -> Result<Json<DbRecord>, String> {
    // insert your application logic here

    dbg!(&payload);

    let user = User::new()
        .set_name(Name::new("Tobias".to_string(), Some("Test".to_string())))
        .set_is_online(true);
    dbg!(&user);

    match db
        .create(DbResource::User.key())
        .content(user.clone())
        .await
    {
        Ok(record) => Ok(Json(record)),
        Err(err) => {
            // log trace of error
            tracing::error!("db error: {:?}", err);
            Err(format!("db error: {:?}", err))
        }
    }
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[allow(dead_code)]
async fn get_users(
    _pagination: Option<Query<Pagination>>,
    State(db): State<Surreal<Client>>,
) -> (StatusCode, Json<Vec<User>>) {
    // insert your application logic here
    let users: Vec<User> = db.select("user").await.unwrap();
    dbg!(users.clone());

    // this will be converted into a JSON response
    // with a status code of `200 OK`
    (StatusCode::OK, Json(users))
}
