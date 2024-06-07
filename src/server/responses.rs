use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;


pub type AppResult<T> = std::result::Result<Json<T>, AppError>;

// The kinds of errors we can hit in our application.
pub enum AppError {
    // The request body contained invalid JSON
    // JsonRejection(JsonRejection),
    JWTEncodingError(jsonwebtoken::errors::Error),
    JWTDecodingError(jsonwebtoken::errors::Error),
    DatabaseQueryError(diesel::result::Error),
    DatabaseConnectionInteractError(deadpool_diesel::InteractError),
    DatabasePoolError(deadpool_diesel::PoolError),
    DoesNotExist,
}

// How we want errors responses to be serialized
#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}

// Tell axum how `AppError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        let (status, message) = match self {
            // AppError::JsonRejection(rejection) => {
            //     (rejection.status(), rejection.body_text())
            // }
            _ => {
                // Because `TraceLayer` wraps each request in a span that contains the request
                // method, uri, etc we don't need to include those details here
                tracing::error!("error from time_library");

                // Don't expose any details about the error to the client
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_owned(),
                )
            }
        };

        (status, Json(ErrorMessage { message })).into_response()
    }
}
