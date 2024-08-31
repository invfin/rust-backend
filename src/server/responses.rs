use std::num::ParseIntError;

use axum::{
    extract::{rejection::JsonRejection, FromRequest},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use maxminddb::MaxMindDBError;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};

pub type AppResult<T> = std::result::Result<Json<T>, AppError>;

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct PaginatedQuery<T> {
    #[serde(flatten)]
    query: T,

    page: Option<i64>,
    per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
pub struct PaginatedResponse<T> {
    data: Vec<T>,
    total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total_pages: i64) -> Self {
        Self { data, total_pages }
    }
    pub fn response(data: Vec<T>, total_pages: i64) -> AppResult<Self> {
        Ok(Json(Self::new(data, total_pages)))
    }
}

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
    //
    WrongPassword(argon2::password_hash::Error),
    ErrorHashingPassword(argon2::password_hash::Error),
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    //
    JWTError(jsonwebtoken::errors::Error),
    JWTModified(ParseIntError),
    //
    DatabaseQueryError(diesel::result::Error),
    DatabaseConnectionInteractError(deadpool_diesel::InteractError),
    DatabasePoolError(deadpool_diesel::PoolError),
    DoesNotExist,
    //
    RoleError,
    //
    UnexpectedError(Box<dyn std::error::Error + Send + Sync>),
    //
    IpError(MaxMindDBError),
    IpDataNotFound,
}

#[derive(Serialize, ToResponse, ToSchema)]
pub struct ErrorMessage {
    #[schema(example = "Sorry no sorry, something wrong happened")]
    message: String,
}

// Tell axum how `AppError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        let (status, message) = match self {
            AppError::ErrorHashingPassword(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            AppError::WrongPassword(_err) => {
                (StatusCode::NOT_FOUND, "Contraseña incorrecta".to_owned())
            }
            //
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),

            AppError::JWTError(err) => (StatusCode::UNAUTHORIZED, err.to_string()),
            AppError::JWTModified(err) => (StatusCode::UNAUTHORIZED, err.to_string()),

            AppError::RoleError => (StatusCode::UNAUTHORIZED, "Not authorized".to_string()),

            AppError::UnexpectedError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),

            AppError::DatabaseQueryError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            AppError::DatabaseConnectionInteractError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            AppError::DatabasePoolError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            AppError::DoesNotExist => (StatusCode::NOT_FOUND, "Not found".to_owned()),

            AppError::IpError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::IpDataNotFound => (StatusCode::INTERNAL_SERVER_ERROR, "Ip wrong".to_owned()),
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
