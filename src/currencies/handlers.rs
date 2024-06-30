use crate::{
    db::schema::{currencies, exchange_rates},
    server::{AppError, AppResult, AppState},
};
use axum::{
    routing::{get},
    Json, Router,
};

use bigdecimal::BigDecimal;
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

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/currencies", get(list_currencies))
        .route(
            "/exchange_rates",
            get(list_exchange_rates),
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
    id: i64,
    base_id: i64,
target_id: i64,
conversion_rate: BigDecimal,
date: chrono::NaiveDateTime,
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
    let conn =  state.db_write().await?;
    let query = conn
        .interact(move |conn| {
            exchange_rates::table
                .select(ExchangeRate::as_select())
                .load::<ExchangeRate>(conn).map_err(AppError::DatabaseQueryError)
                
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??;

        Ok(Json(query))
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
    let conn =  state.db_write().await?;
    let query = conn
        .interact(move |conn| {
            currencies::table
                .select(Currency::as_select())
                .load::<Currency>(conn).map_err(AppError::DatabaseQueryError)
                
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??;

        Ok(Json(query))
}
