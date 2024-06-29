use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

pub type AppResult<T> = std::result::Result<Json<T>, AppError>;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[derive(Debug)]
pub enum AppError {
    WrongPassword(argon2::password_hash::Error),
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    JWTError(jsonwebtoken::errors::Error),
    DatabaseQueryError(diesel::result::Error),
    DatabaseConnectionInteractError(deadpool_diesel::InteractError),
    DatabasePoolError(deadpool_diesel::PoolError),
    DoesNotExist,
}

// How we want errors responses to be serialized
#[derive(Serialize, ToResponse, ToSchema)]
pub struct ErrorMessage {
    message: String,
}

// Tell axum how `AppError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            AppError::JWTError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::DatabaseQueryError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            AppError::DatabaseConnectionInteractError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            AppError::DatabasePoolError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            AppError::WrongPassword(err) => {
                (StatusCode::NOT_FOUND, "Contraseña incorrecta".to_owned())
            }
            AppError::DoesNotExist => (StatusCode::NOT_FOUND, "Not found".to_owned()),
        };

        (status, Json(ErrorMessage { message })).into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Self::JWTError(error)
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> Self {
        Self::DatabaseQueryError(error)
    }
}

impl From<deadpool_diesel::InteractError> for AppError {
    fn from(error: deadpool_diesel::InteractError) -> Self {
        Self::DatabaseConnectionInteractError(error)
    }
}

impl From<deadpool_diesel::PoolError> for AppError {
    fn from(error: deadpool_diesel::PoolError) -> Self {
        Self::DatabasePoolError(error)
    }
}
