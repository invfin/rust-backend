use crate::{
    db::schema::exchanges,
    server::{AppError, AppResult},
    AppState,
};

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use utoipa::{self, OpenApi, ToResponse, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(create_exchange, delete_exchange, read_exchange, update_exchange, list_exchanges),
    components(schemas(Exchange),
    responses(Exchange)),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/exchanges", post(create_exchange).get(list_exchanges))
        .route(
            "/exchanges/:id",
            get(read_exchange)
                .put(update_exchange)
                .delete(delete_exchange),
        )
        .with_state(state)
}

#[derive(
    Queryable, Insertable, AsChangeset, Serialize, Deserialize, Selectable, ToSchema, ToResponse,
)]
#[diesel(table_name = exchanges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Exchange {
    id: i64,
    name: String,
    ticker: String,
    country_id: Option<i64>,
    image: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    get,
    path = "exchanges",
    responses(
            (status = 200, body = Vec<Exchange>, description = "A paginated result of exchanges with key information"),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
)]
async fn list_exchanges(state: AppState) -> AppResult<Vec<Exchange>> {
    let conn = state.db_write().await?;
    let query = conn
        .interact(move |conn| {
            exchanges::table
                .select(Exchange::as_select())
                .load::<Exchange>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(query))
}

#[utoipa::path(
    post,
    path = "exchanges",
    request_body = Exchange,
    responses(
        (status = 200, body = Exchange, description = "Create a new exchange"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn create_exchange(state: AppState, Json(exchange): Json<Exchange>) -> AppResult<Exchange> {
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
    path = "exchanges/{id}",
    params(("id" = i64, Path, description = "Exchange ID")),
    responses(
        (status = 200, body = Exchange, description = "Read an exchange by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn read_exchange(Path(id): Path<i64>, state: AppState) -> AppResult<Exchange> {
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
    path = "exchanges/{id}",
    params(("id" = i64, Path, description = "Exchange ID")),
    request_body = Exchange,
    responses(
        (status = 200, body = Exchange, description = "Update an exchange by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn update_exchange(
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
    path = "exchanges/{id}",
    params(("id" = i64, Path, description = "Exchange ID")),
    responses(
        (status = 200, body = usize, description = "Delete an exchange by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn delete_exchange(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
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
