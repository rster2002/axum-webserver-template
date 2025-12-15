#[macro_use]
extern crate tracing;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use axum::http::{header, Method, StatusCode};
use axum::{BoxError, Json, Router};
use axum::error_handling::HandleErrorLayer;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use crate::app_state::AppState;
use crate::errors::dto_error::ErrorDto;
use crate::init::StartOptions;
use crate::modules::health::create_health_router;


mod init;
mod app_state;
mod shared;
mod modules;
mod errors;

#[tokio::main]
async fn main() {
    let env_result = dotenv::dotenv();

    match env_result {
        Err(dotenv::Error::Io(_)) => {}
        Err(err) => panic!("{:?}", err),
        _ => {}
    }

    tracing_subscriber::fmt::init();

    trace!("Creating start options");
    let start_options = StartOptions::from_env();

    trace!("Creating collection pool");
    let pool = PgPoolOptions::new()
        .max_connections(start_options.max_db_connections)
        .connect(&start_options.database_url)
        .await
        .expect("Could not create database pool");

    trace!("Running migrations");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to migrate");

    trace!("Creating memcached client");
    let memcached_client = memcached::Client::connect(start_options.memcached_url.clone())
        .expect("Failed to connect to memcached client");

    let app_state = AppState {
        start_options: start_options.clone(),
        pool: Arc::new(pool),
        memcached_client,
    };

    let request_buffer = BufferLayer::new(start_options.request_buffer_size);

    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_headers([
            header::CONTENT_TYPE,
            header::CONTENT_LENGTH,
            header::CONTENT_ENCODING,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(start_options.cors_origins.clone());

    let global_rate_limiter = RateLimitLayer::new(
        start_options.global_limit_requests,
        Duration::from_secs(start_options.global_limit_per_seconds),
    );

    let layers = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                #[cfg(not(debug_assertions))]
                "Internal server error".to_string(),

                #[cfg(debug_assertions)]
                format!("Uncaught error: {}", err.to_string()),
            )
        }))
        .layer(request_buffer)
        .layer(global_rate_limiter);

    let router = Router::new()
        .merge(create_health_router())
        .with_state(app_state)
        .layer(layers)
        .layer(cors);

    let listener = TcpListener::bind(&start_options.bind_address)
        .await
        .expect("Failed to bind to TCP listener");

    info!("Running server...");
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to start server")
}

async fn handle_error(err: BoxError) -> (StatusCode, String) {
    (
        StatusCode::BAD_REQUEST,
        format!("oh noes error! {err:?}"),
    )
}
