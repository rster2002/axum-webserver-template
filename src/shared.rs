use std::sync::Arc;
use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;
pub type SharedPool = Arc<DbPool>;
