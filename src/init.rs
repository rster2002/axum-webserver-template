use std::env;
use std::net::SocketAddr;
use axum::http::HeaderValue;
use url::Url;

#[derive(Debug, Clone)]
pub struct StartOptions {
    pub bind_address: SocketAddr,
    pub server_url: Url,
    pub database_url: String,
    pub max_db_connections: u32,
    pub memcached_url: String,
    pub secure_proxy: bool,
    pub cors_origins: Vec<HeaderValue>,
    pub global_limit_requests: u64,
    pub global_limit_per_seconds: u64,
    pub request_buffer_size: usize,
}

impl StartOptions {
    pub fn from_env() -> Self {
        let bind_address = env::var("BIND_ADDRESS")
            .expect("BIND_ADDRESS not set")
            .parse()
            .expect("BIND_ADDRESS not a SocketAddr");

        let server_url: Url = env::var("SERVER_URL")
            .expect("SERVER_URL not set")
            .parse()
            .expect("Failed to parse SERVER_URL");

        if server_url.host().is_none() {
            panic!("SERVER_URL did not contain a host");
        }

        let db_connection_string = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let max_db_connections = env::var("MAX_DB_CONNECTIONS")
            .expect("MAX_DB_CONNECTIONS not set")
            .parse()
            .expect("MAX_DB_CONNECTIONS not a u32");

        let memcached_url = env::var("MEMCACHED_URL")
            .expect("MEMCACHED_URL not set");

        let secure_proxy_string = env::var("SECURE_PROXY").expect("SECURE_PROXY not set");

        let secure_proxy = match &*secure_proxy_string {
            "true" => true,
            "false" => false,
            _ => panic!("SECURE_PROXY must either be 'true' or 'false'"),
        };

        let cors_origins = env::var("CORS_ORIGINS")
            .expect("CORS_ORIGINS not set")
            .split(',')
            .map(|cors_str| {
                cors_str
                    .parse()
                    .expect("Failed to parse HeaderValue for CORS_ORIGINS")
            })
            .collect();

        let global_limit_requests = env::var("GLOBAL_LIMIT_REQUESTS")
            .expect("GLOBAL_LIMIT_REQUESTS not set")
            .parse()
            .expect("GLOBAL_LIMIT_REQUESTS not a u64");

        let global_limit_per_seconds = env::var("GLOBAL_LIMIT_PER_SECONDS")
            .expect("GLOBAL_LIMIT_PER_SECONDS not set")
            .parse()
            .expect("GLOBAL_LIMIT_PER_SECONDS not a u64");

        let request_buffer_size = env::var("REQUEST_BUFFER_SIZE")
            .expect("REQUEST_BUFFER_SIZE not set")
            .parse()
            .expect("REQUEST_BUFFER_SIZE not a usize");

        Self {
            bind_address,
            server_url,
            database_url: db_connection_string,
            max_db_connections,
            memcached_url,
            secure_proxy,
            cors_origins,
            global_limit_requests,
            global_limit_per_seconds,
            request_buffer_size,
        }
    }
}
