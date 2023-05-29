mod db_client;
mod models;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Result,
    routing::{delete, get},
    Json, Router,
};
use db_client::{Db, DbRecord};
use models::{user::User, ApiUserCreateRequest};
use serde::Deserialize;
use std::net::SocketAddr;
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal,
};
use validator::Validate;

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
        .route("/user", get(get_users).post(create_user))
        .route("/user/:id", delete(delete_user).get(get_user))
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
    Json(payload): Json<ApiUserCreateRequest>,
) -> Result<Json<DbRecord>, String> {
    // insert your application logic here

    dbg!(&payload);

    match payload.validate() {
        Ok(_) => {
            println!("create user payload ok");
        }
        Err(err) => {
            // log trace of error
            tracing::error!("validation error: {:?}", err);
            return Err(format!("validation error: {:?}", err));
        }
    };

    let user = User::new(payload);

    dbg!(&user);

    match db
        .create((DbResource::User.key(), &user.id))
        .content(&user)
        .await
    {
        Ok(record) => {
            println!("db created user {:?}", user);

            Ok(Json(record))
        }
        Err(err) => {
            // log trace of error
            tracing::error!("db error: {:?}", err);
            Err(format!("db error: {:?}", err))
        }
    }
}

#[axum_macros::debug_handler]
async fn delete_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(db): State<Surreal<Client>>,
    Path(id): Path<String>,
) -> Result<Json<DbRecord>, String> {
    // insert your application logic here
    dbg!("delete_user", &id);
    let id_valid = uuid::Uuid::parse_str(&id);

    if id_valid.is_err() {
        tracing::error!("can not delete user without valid uuidv4");
        return Err("can not delete user without valid uuidv4".to_string());
    }

    println!("delete user id ok");

    // let _id = DbRecord {
    //     id: Thing {
    //         tb: "id".to_string(),
    //         id: Id::String(id_valid.unwrap().to_string()),
    //     },
    // };

    let deleted: Vec<User> = db.delete("user").await.unwrap_or_default();

    dbg!(&deleted);

    match deleted.len().le(&1) {
        true => Ok(Json(DbRecord {
            id: Thing {
                tb: "user".to_string(),
                id: Id::String(id_valid.unwrap().to_string()),
            },
        })),
        false => Err("can not delete user".to_string()),
    }
    // match id.as_str().is_empty() {
    //     Ok(_) => {
    //         println!("create user payload ok");
    //     }
    //     Err(err) => {
    //         // log trace of error
    //         tracing::error!("validation error: {:?}", err);
    //         return Err(format!("validation error: {:?}", err));
    //     }
    // };

    // let user = User::new(payload);

    // dbg!(&user);

    // match db
    //     .create(DbResource::User.key())
    //     .content(user.clone())
    //     .await
    // {
    //     Ok(record) => Ok(Json(record)),
    //     Err(err) => {
    //         // log trace of error
    //         tracing::error!("db error: {:?}", err);
    //         Err(format!("db error: {:?}", err))
    //     }
    // }
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

// insert your application logic here
#[axum_macros::debug_handler]
async fn get_users(
    _pagination: Option<Query<Pagination>>,
    State(db): State<Surreal<Client>>,
) -> (StatusCode, Json<Option<Vec<User>>>) {
    match db.select("user").await {
        Ok(users) => {
            dbg!(&users);
            (StatusCode::OK, Json(Some(users)))
        }
        Err(err) => {
            dbg!(&err);

            (StatusCode::NOT_FOUND, Json(None))
        }
    }
}

#[axum_macros::debug_handler]
async fn get_user(
    State(db): State<Surreal<Client>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Option<User>>) {
    let resource = (DbResource::User.key().to_string(), id.to_string());
    println!("get_user({:?})", &resource);

    match db.select(resource).await {
        Ok(user) => {
            dbg!(&user);

            (StatusCode::OK, Json(user))
        }
        Err(err) => {
            tracing::error!("db error: {:?}", err);
            (StatusCode::NOT_FOUND, Json(None))
        }
    }
}
