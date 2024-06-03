/// Implements the [axum::response::IntoResponse] trait for the given type. The given type should
/// be an error type and also provide a method called `status` which returns a
/// [axum::http::StatusCode]. Implementing the [crate::errors::dto_error::DtoError] trait will make
/// sure you meet these criteria.
#[macro_export]
macro_rules! impl_error_response {
    ($ty:ty) => {
        use axum::http::header;
        use axum::response::{IntoResponse, Response};
        use $crate::errors::dto_error::ErrorDto;

        impl IntoResponse for $ty {
            fn into_response(self) -> Response {
                let status = self.status();
                let mut message = self.to_string();

                if matches!(status, StatusCode::INTERNAL_SERVER_ERROR) {
                    error!("Internal server error: {}", &message);

                    #[cfg(not(debug_assertions))]
                    {
                        message = "Internal server error".to_string();
                    }
                }

                let body_result = serde_json::to_string(&ErrorDto {
                    status: status.as_u16(),
                    message,
                });

                let Ok(body_string) = body_result else {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                };

                (
                    status,
                    [(header::CONTENT_TYPE, "application/json")],
                    body_string,
                )
                    .into_response()
            }
        }
    };
}
