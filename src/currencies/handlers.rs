use crate::{AppError, AppResult, AppState};
use crate::{AppError, AppResult, AppState};
use axum::{extract::Path, Json};
use axum::{extract::Path, Json};
use diesel::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = currencies]
pub struct Currency {
    pub id: i64,
    pub name: String,
    pub alphabetic_code: String,
    pub numeric_code: String,
    pub symbol: String,
    pub decimals: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/currencies",
    request_body = Currency,
    responses(
        (status = 200, body = Currency, description = "Create a new currency"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_currency(
    state: AppState,Json(currency): Json<Currency>,
    
) -> AppResult<Currency> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(currencies::table)
                .values(&currency)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/currencies/{id}",
    params(
        ("id" = i64, Path, description = "Currency ID")
    ),
    responses(
        (status = 200, body = Currency, description = "Read a currency by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_currency(Path(id): Path<i64>, state: AppState) -> AppResult<Currency> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            currencies::table
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
    path = "/currencies/{id}",
    params(
        ("id" = i64, Path, description = "Currency ID")
    ),
    request_body = Currency,
    responses(
        (status = 200, body = Currency, description = "Update a currency by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_currency(
    Path(id): Path<i64>,
    state: AppState,Json(currency): Json<Currency>,
    
) -> AppResult<Currency> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(currencies::table.find(id))
                .set(&currency)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/currencies/{id}",
    params(
        ("id" = i64, Path, description = "Currency ID")
    ),
    responses(
        (status = 200, description = "Delete a currency by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn delete_currency(Path(id): Path<i64>, state: AppState) -> AppResult<()> {
    let conn = state.db_write().await?;
    conn.interact(move |conn| {
        diesel::delete(currencies::table.find(id))
            .execute(conn)
            .map_err(AppError::DatabaseQueryError)
    })
    .await
    .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(())
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = currencies_countries_m2m]
pub struct CurrencyCountryM2M {
    pub id: i64,
    pub currency_id: i64,
    pub country_id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/currencies_countries_m2m",
    request_body = CurrencyCountryM2M,
    responses(
        (status = 200, body = CurrencyCountryM2M, description = "Create a new currency-country relation"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_currency_country_m2m(
    state: AppState,Json(currency_country_m2m): Json<CurrencyCountryM2M>,
    
) -> AppResult<CurrencyCountryM2M> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(currencies_countries_m2m::table)
                .values(&currency_country_m2m)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/currencies_countries_m2m/{id}",
    params(
        ("id" = i64, Path, description = "CurrencyCountryM2M ID")
    ),
    responses(
        (status = 200, body = CurrencyCountryM2M, description = "Read a currency-country relation by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_currency_country_m2m(
    Path(id): Path<i64>,
    state: AppState,
) -> AppResult<CurrencyCountryM2M> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            currencies_countries_m2m::table
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
    path = "/currencies_countries_m2m/{id}",
    params(
        ("id" = i64, Path, description = "CurrencyCountryM2M ID")
    ),
    request_body = CurrencyCountryM2M,
    responses(
        (status = 200, body = CurrencyCountryM2M, description = "Update a currency-country relation by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_currency_country_m2m(
    Path(id): Path<i64>,
    state: AppState,Json(currency_country_m2m): Json<CurrencyCountryM2M>,
    
) -> AppResult<CurrencyCountryM2M> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(currencies_countries_m2m::table.find(id))
                .set(&currency_country_m2m)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/currencies_countries_m2m/{id}",
    params(
        ("id" = i64, Path, description = "CurrencyCountryM2M ID")
    ),
    responses(
        (status = 200, description = "Delete a currency-country relation by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn delete_currency_country_m2m(Path(id): Path<i64>, state: AppState) -> AppResult<()> {
    let conn = state.db_write().await?;
    conn.interact(move |conn| {
        diesel::delete(currencies_countries_m2m::table.find(id))
            .execute(conn)
            .map_err(AppError::DatabaseQueryError)
    })
    .await
    .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(())
}
use crate::{AppError, AppResult, AppState};
use axum::{extract::Path, Json};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = countries]
pub struct Country {
    pub id: i64,
    pub name: String,
    pub iso: String,
    pub alpha_2_code: String,
    pub alpha_3_code: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/countries",
    request_body = Country,
    responses(
        (status = 200, body = Country, description = "Create a new country"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_country(Json(country): Json<Country>, state: AppState) -> AppResult<Country> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(countries::table)
                .values(&country)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/countries/{id}",
    params(
        ("id" = i64, Path, description = "Country ID")
    ),
    responses(
        (status = 200, body = Country, description = "Read a country by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_country(Path(id): Path<i64>, state: AppState) -> AppResult<Country> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            countries::table
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
    path = "/countries/{id}",
    params(
        ("id" = i64, Path, description = "Country ID")
    ),
    request_body = Country,
    responses(
        (status = 200, body = Country, description = "Update a country by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_country(
    Path(id): Path<i64>,
    state: AppState,Json(country): Json<Country>,
    
) -> AppResult<Country> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(countries::table.find(id))
                .set(&country)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/countries/{id}",
    params(
        ("id" = i64, Path, description = "Country ID")
    ),
    responses(
        (status = 200, description = "Delete a country by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn delete_country(Path(id): Path<i64>, state: AppState) -> AppResult<()> {
    let conn = state.db_write().await?;
    conn.interact(move |conn| {
        diesel::delete(countries::table.find(id))
            .execute(conn)
            .map_err(AppError::DatabaseQueryError)
    })
    .await
    .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(())
}

pub fn router() -> Router {
    Router::new()
        // Countries
        .route("/countries", post(create_country).get(list_countries))
        .route(
            "/countries/:id",
            get(read_country).put(update_country).delete(delete_country),
        )
        // Currencies
        .route("/currencies", post(create_currency).get(list_currencies))
        .route(
            "/currencies/:id",
            get(read_currency)
                .put(update_currency)
                .delete(delete_currency),
        )
        // Currencies-Countries M2M
        .route(
            "/currencies_countries_m2m",
            post(create_currency_country_m2m).get(list_currency_country_m2m),
        )
        .route(
            "/currencies_countries_m2m/:id",
            get(read_currency_country_m2m)
                .put(update_currency_country_m2m)
                .delete(delete_currency_country_m2m),
        )
        // Exchange Rates
        .route(
            "/exchange_rates",
            post(create_exchange_rate).get(list_exchange_rates),
        )
        .route(
            "/exchange_rates/:id",
            get(read_exchange_rate)
                .put(update_exchange_rate)
                .delete(delete_exchange_rate),
        )
}
