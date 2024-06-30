use crate::{
    db::schema::exchanges,
    server::{AppError, AppResult},
    AppState,
};
use axum::{extract::Path, Json};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::{self, ToResponse};
use utoipa::OpenApi;
use utoipa::ToSchema;

#[derive(OpenApi)]
#[openapi(
        paths(
            create_exchange, delete_exchange, read_exchange, update_exchange
        ),
        components(schemas(Exchange), 
    responses(Exchange)),
    security(
        ("token_jwt" = [])
    )
    )]
pub struct ApiDoc;


#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, ToSchema, ToResponse)]
#[diesel(table_name = exchanges)]
pub struct Exchange {
    pub id: i64,
    pub name: String,
    pub ticker: String,
    pub country_id: Option<i64>,
    pub image: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/exchanges",
    request_body = Exchange,
    responses(
        (status = 200, body = Exchange, description = "Create a new exchange"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_exchange(
    state: AppState,
    Json(exchange): Json<Exchange>,
) -> AppResult<Exchange> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(exchanges::table)
                .values(&exchange)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/exchanges/{id}",
    params(
        ("id" = i64, Path, description = "Exchange ID")
    ),
    responses(
        (status = 200, body = Exchange, description = "Read an exchange by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_exchange(Path(id): Path<i64>, state: AppState) -> AppResult<Exchange> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            exchanges::table
                .find(id)
                .first(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/exchanges/{id}",
    params(
        ("id" = i64, Path, description = "Exchange ID")
    ),
    request_body = Exchange,
    responses(
        (status = 200, body = Exchange, description = "Update an exchange by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_exchange(
    Path(id): Path<i64>,
    state: AppState,
    Json(exchange): Json<Exchange>,
) -> AppResult<Exchange> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(exchanges::table.find(id))
                .set(&exchange)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/exchanges/{id}",
    params(
        ("id" = i64, Path, description = "Exchange ID")
    ),
    responses(
        (status = 200, body = usize, description = "Delete an exchange by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn delete_exchange(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
    let conn = state.db_write().await?;
    Ok(Json(
        conn.interact(move |conn| {
            diesel::delete(exchanges::table.find(id))
                .execute(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}
