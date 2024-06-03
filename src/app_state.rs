use std::sync::Arc;
use axum::extract::FromRef;
use memcached::Client;
use crate::init::StartOptions;
use crate::shared::{DbPool, SharedPool};

#[derive(Clone)]
pub struct AppState {
    pub start_options: StartOptions,
    pub pool: Arc<DbPool>,
    pub memcached_client: Client,
}

impl FromRef<AppState> for SharedPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

impl FromRef<AppState> for StartOptions {
    fn from_ref(input: &AppState) -> Self {
        input.start_options.clone()
    }
}

impl FromRef<AppState> for Client {
    fn from_ref(input: &AppState) -> Self {
        input.memcached_client.clone()
    }
}
