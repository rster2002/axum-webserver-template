use axum::http::StatusCode;
use serde::Serialize;

pub trait DtoError: std::error::Error {
    fn status(&self) -> StatusCode;
}

#[derive(Debug, Serialize)]
pub struct ErrorDto {
    pub status: u16,
    pub message: String,
}
