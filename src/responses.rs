use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use serde_json::{json, Value};
use std::fmt::Debug;

#[derive(Debug)]
pub enum CRUDError {
    NotFound,
    MaxRetry,
    WrongParameters,
    Write,
    Delete,
    JsonError,
    InternalError,
}

pub async fn success<T: serde::Serialize>(data: T) -> Response {
    (StatusCode::OK, Json(json!({ "data": data }))).into_response()
}

pub async fn non_auth() -> Response {
    (StatusCode::FORBIDDEN, Json(json!({"message": "Not auth"}))).into_response()
}

pub async fn max_limit() -> Response {
    (
        StatusCode::NOT_ACCEPTABLE,
        Json(json!({"message": "limit exceeded"})),
    )
        .into_response()
}
pub async fn not_found<T: Debug>(data: &T) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "message": format!("{:#?} not found", data) })),
    )
        .into_response()
}
pub async fn wrong_query<T: Debug + ?Sized>(query: &T) -> Response {
    pre_wrong_query(query).await.into_response()
}
pub async fn pre_wrong_query<T: Debug + ?Sized>(query: &T) -> (StatusCode, axum::Json<Value>) {
    (
        StatusCode::NOT_ACCEPTABLE,
        Json(json!({ "message": format!("{:#?}", query) })),
    )
}
pub async fn our_fault() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"message": "oupsi"})),
    )
        .into_response()
}

pub async fn match_error<T: Serialize + Send, P: Serialize + Send + Debug>(
    result: Result<T, CRUDError>,
    params: &P,
) -> Response {
    match result {
        Ok(u) => success(u).await,
        Err(err) => match err {
            CRUDError::NotFound => not_found(params).await,
            CRUDError::MaxRetry => max_limit().await,
            CRUDError::WrongParameters => not_found(params).await,
            CRUDError::Write => our_fault().await,
            CRUDError::Delete => our_fault().await,
            CRUDError::JsonError => our_fault().await,
            CRUDError::InternalError => our_fault().await,
        },
    }
}
