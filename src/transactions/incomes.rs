use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use bigdecimal::BigDecimal;
use diesel::{
    AsChangeset, Insertable, QueryDsl, Queryable,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::{
        schema::{ expenses, incomes, investments},
        Paginate, Paginated,
    },
    server::{AppError,  AppResult, AppState},
};
use chrono::NaiveDateTime;

use utoipa::{self, ToResponse};
use utoipa::OpenApi;
use utoipa::ToSchema;

#[derive(OpenApi)]
#[openapi(
        paths(
            create_income, get_income, update_income, delete_income,
        ),
        components(schemas(Income,  IncomeResponse), 
    responses(IncomeResponse)),
    security(
        ("token_jwt" = [])
    )
    )]
pub struct ApiDoc;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Income {
    user_id: i64,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    amount: BigDecimal,
    #[schema(value_type = NaiveDateTime, example = NaiveDateTime::default)]
    date: NaiveDateTime,
    #[schema(example = "The description or null")]
    description: Option<String>,
    #[schema(example = "The comment or null")]
    comment: Option<String>,
    currency_id: i64,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    amount_converted: BigDecimal,
}

#[derive(Debug, Serialize, Deserialize,ToResponse, Queryable, Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = incomes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct IncomeResponse {
    id: i64,
    user_id: i64,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    amount: BigDecimal,
    #[schema(value_type = NaiveDateTime, example = NaiveDateTime::default)]
    date: NaiveDateTime,
    #[schema(example = "The description or null")]
    description: Option<String>,
    #[schema(example = "The comment or null")]
    comment: Option<String>,
    currency_id: i64,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    amount_converted: BigDecimal,
    #[schema(value_type = NaiveDateTime, example = NaiveDateTime::default)]
    created_at: NaiveDateTime,
    #[schema(value_type = NaiveDateTime, example = NaiveDateTime::default)]
    updated_at: NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "incomes",
    tag = "Incomes",
    request_body = Income,
    responses(
        (status = 201, body = IncomeResponse, description = "Created income"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn create_income(
    state: AppState,
    Json(new_income): Json<Income>,
) -> AppResult<IncomeResponse> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(incomes::table)
                .values(&new_income)
                .get_result::<IncomeResponse>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "incomes/{id}",
    tag = "Incomes",
    request_body = Income,
    responses(
        (status = 200, body = IncomeResponse, description = "Updated income record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn update_income(
    Path(id): Path<i64>,
    state: AppState,
    Json(updated_income): Json<Income>,
) -> AppResult<IncomeResponse> {
    let conn = state.db_write().await?;

    let query = conn
        .interact(move |conn| {
            diesel::update(incomes::table.find(id))
                .set(&updated_income)
                .get_result::<IncomeResponse>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(query))
}

#[utoipa::path(
    get,
    path = "incomes/{id}",
    tag = "Incomes",
    responses(
        (status = 200, body = IncomeResponse, description = "The income record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn get_income(Path(id): Path<i64>, state: AppState) -> AppResult<IncomeResponse> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            incomes::table
                .find(id)
                .first::<IncomeResponse>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "incomes/{id}",
    tag = "Incomes",
    responses(
        (status = 204, body = usize, description = "Deleted income record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn delete_income(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::delete(incomes::table.find(id))
                .execute(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(result))
}

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/incomes", post(create_income))
        .route(
            "/incomes/:id",
            get(get_income).put(update_income).delete(delete_income),
        )
        .with_state(state)
}
