use crate::{
    db::schema::{currencies, exchange_rates},
    server::{AppError, AppResult, AppState},
};
use axum::{
    routing::{get},
    Json, Router,
};

use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use utoipa::{self,OpenApi,ToSchema, ToResponse};

#[derive(OpenApi)]
#[openapi(
    paths(list_exchange_rates, list_currencies),
    components(schemas(Currency, ExchangeRate), 
    responses(Currency, ExchangeRate)),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(_state: AppState) -> Router<AppState> {
    Router::new()
        .route("/currencies", get(list_currencies))
        .route(
            "/exchange_rates",
            get(list_exchange_rates).post(create_exchange_rate),
        )
}

#[derive(Queryable, Serialize,Selectable, Deserialize, ToSchema, ToResponse)]
#[diesel(table_name = currencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Currency {
    id: i64,
name: String,
alphabetic_code: String,
numeric_code: String,
symbol: String,
decimals: i32,
}

#[derive(Queryable, Serialize,Selectable, Deserialize, ToSchema, ToResponse)]
#[diesel(table_name = exchange_rates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct ExchangeRate {
    base_id: i64,
    target_id: i64,
    conversion_rate: String,
    precision: i32,
    scale: i32,
    date: chrono::NaiveDate,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct ExchangeRatePayload {
    base: String,
    target: String,
    conversion_rate: String,
    precision: i32,
    scale: i32,
    date: chrono::NaiveDate,
    source: String
}

#[utoipa::path(
    post,
    path = "exchange_rates",
    request_body = ExchangeRatePayload,
    responses(
        (status = 200, body = ExchangeRate, description = "Create a new exchange rate"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn create_exchange_rate(state: AppState, Json(payload): Json<ExchangeRatePayload>) -> AppResult<ExchangeRate> {
    Ok(Json(state.db_write().await?
    .interact(move |conn| {
        let target_id: i64 = currencies::table
        .filter(currencies::alphabetic_code.eq(payload.target))
            .select(currencies::id)
            .first(conn).map_err(AppError::DatabaseQueryError)?;

            let base_id: i64 = currencies::table
            .filter(currencies::alphabetic_code.eq(payload.base))
                .select(currencies::id)
                .first(conn).map_err(AppError::DatabaseQueryError)?;

        diesel::insert_into(exchange_rates::table)
        .values((
            exchange_rates::conversion_rate.eq(payload.conversion_rate), 
            exchange_rates::base_id.eq(base_id),
            exchange_rates::target_id.eq(target_id),
            exchange_rates::date.eq(payload.date),
            exchange_rates::precision.eq(payload.precision),
            exchange_rates::scale.eq(payload.scale),
            exchange_rates::source.eq(payload.source)
        ))
        .returning(ExchangeRate::as_returning())
            .get_result(conn)
            .map_err(AppError::DatabaseQueryError)
    })
    .await
    .map_err(AppError::DatabaseConnectionInteractError)??))
}



#[utoipa::path(
    get, 
    path = "exchange_rates",
    responses(
            (status = 200, body = Vec<ExchangeRate>, description = "A paginated result of exchange_rates with key information"),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
        
)]
async fn list_exchange_rates(state: AppState) -> AppResult<Vec<ExchangeRate>> {
        Ok(Json(state.db_write().await?
        .interact(move |conn| {
            exchange_rates::table
                .select(ExchangeRate::as_select())
                .load::<ExchangeRate>(conn).map_err(AppError::DatabaseQueryError)
                
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??))
}

#[utoipa::path(
    get, 
    path = "currencies",
    responses(
            (status = 200, body = Vec<Currency>, description = "A paginated result of currencies with key information"),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
        
)]
async fn list_currencies(state: AppState) -> AppResult<Vec<Currency>> {
        Ok(Json(state.db_write().await?
        .interact(move |conn| {
            currencies::table
                .select(Currency::as_select())
                .load::<Currency>(conn).map_err(AppError::DatabaseQueryError)
                
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??))
}
