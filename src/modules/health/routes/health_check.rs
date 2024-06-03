use axum::extract::State;
use axum::http::StatusCode;
use memcached::Client;
use crate::shared::SharedPool;

pub async fn health_check(
    State(memcached): State<Client>,
    State(pool): State<SharedPool>,
) -> StatusCode {
    let memcached_up = memcached.stats()
        .await
        .is_ok();

    let db_up = pool.acquire()
        .await
        .is_ok();

    if memcached_up && db_up {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
