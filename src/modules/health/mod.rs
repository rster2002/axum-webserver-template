pub mod routes;

use axum::Router;
use axum::routing::get;
use crate::app_state::AppState;
use crate::modules::health::routes::health_check::health_check;

pub fn create_health_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
}
