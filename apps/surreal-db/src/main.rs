mod db_client;
mod models;

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Query, State},
    http::StatusCode,
    response::Result,
    routing::{get, post},
    Json, Router,
};
use db_client::{Db, DbRecord};
use models::{user::User, Name};
use serde::Deserialize;
use std::{net::SocketAddr, time::Duration};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::{configure_log, setup_config};

mod utils;

use crate::models::DbResource;
use crate::utils::Settings;

#[tokio::main]
async fn main() {
    let config: Settings = setup_config().expect("could not setup config");
    configure_log(config.log.level);
    info!("starting axum server");
    // initialize tracing
    tracing_subscriber::registry()
        // .with(
        //     tracing_subscriber::EnvFilter::try_from_default_env()
        //         .unwrap_or_else(|_| "users=debug,tower_http=debug".into()),
        // )
        // .with(tracing_subscriber::fmt::layer())
        .init();
    info!("tracing initialized");

    let db = match Db::new().await {
        Ok(db) => db,
        Err(err) => {
            tracing::error!("db error: {:?}", err);
            return;
        }
    };

    // ✅ connect db client
    // ✅ pass db client to route state
    // ✅ use db client in route handler to get user
    // 4. use db client in route handler to create user
    // 5. test get users
    // 6. test get users with pagination
    // 7. test create users
    // 8. save api calls
    // build our application with a route

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/user/list", get(root))
        // `POST /users` goes to `create_user`
        .route("/user/create", post(create_user))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db.client);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
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
    let config: Settings = setup_config().expect("could not setup config");
    configure_log(config.log.level);
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
            return Err(format!("db error: {:?}", err));
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
