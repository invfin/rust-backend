use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, OptionalExtension, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::{schema::companies, Paginate, Paginated},
    server::{AppError, AppJson, AppResult, AppState},
};
use chrono::NaiveDate;

use utoipa;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa::{IntoParams, ToResponse, ToSchema};

#[derive(OpenApi)]
#[openapi(
        paths(
            create_income, get_income, update_income, delete_income,
            create_expense, get_expense, update_expense, delete_expense,
            create_investment, get_investment, update_investment, delete_investment
        ),
        components(schemas(Income, Expense, Investment)),
        tags(
            (name = "Incomes", description = "Income operations"),
            (name = "Expenses", description = "Expense operations"),
            (name = "Investments", description = "Investment operations")
        )
    )]
pub struct ApiDoc;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ToSchema)]
pub struct Income {
    pub id: i64,
    pub user_id: i64,
    pub amount: BigDecimal,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub currency_id: i64,
    pub amount_converted: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ToSchema)]
pub struct Expense {
    pub id: i64,
    pub user_id: i64,
    pub amount: BigDecimal,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub currency_id: Option<i64>,
    pub amount_converted: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ToSchema)]
pub struct Investment {
    pub id: i64,
    pub fee: BigDecimal,
    pub quantity: BigDecimal,
    pub cost: BigDecimal,
    pub user_id: i64,
    pub amount: BigDecimal,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub currency_id: i64,
    pub amount_converted: BigDecimal,
    pub asset_type: String,
    pub asset_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Create Income
#[utoipa::path(
    post,
    path = "incomes",
    request_body = Income,
    responses(
        (status = 201, body = Income, description = "Created income"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
pub async fn create_income(
    Json(new_income): Json<Income>,
    state: AppState,
) -> AppResult<Json<Income>> {
    let conn = get_conn(state).await?;
    let result = diesel::insert_into(incomes::table)
        .values(&new_income)
        .get_result::<Income>(&conn)
        .map_err(AppError::DatabaseQueryError)?;
    Ok(Json(result))
}

// Read Income
#[utoipa::path(
    get,
    path = "incomes/{id}",
    responses(
        (status = 200, body = Income, description = "The income record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
pub async fn get_income(Path(id): Path<i64>, state: AppState) -> AppResult<Json<Income>> {
    let conn = get_conn(state).await?;
    let result = incomes::table
        .find(id)
        .first::<Income>(&conn)
        .map_err(AppError::DatabaseQueryError)?;
    Ok(Json(result))
}

// Update Income
#[utoipa::path(
    put,
    path = "incomes/{id}",
    request_body = Income,
    responses(
        (status = 200, body = Income, description = "Updated income record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
pub async fn update_income(
    Path(id): Path<i64>,
    Json(updated_income): Json<Income>,
    state: AppState,
) -> AppResult<Json<Income>> {
    let conn = get_conn(state).await?;
    let result = diesel::update(incomes::table.find(id))
        .set(&updated_income)
        .get_result::<Income>(&conn)
        .map_err(AppError::DatabaseQueryError)?;
    Ok(Json(result))
}

// Delete Income
#[utoipa::path(
    delete,
    path = "incomes/{id}",
    responses(
        (status = 204, description = "Deleted income record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
pub async fn delete_income(Path(id): Path<i64>, state: AppState) -> AppResult<()> {
    let conn = get_conn(state).await?;
    diesel::delete(incomes::table.find(id))
        .execute(&conn)
        .map_err(AppError::DatabaseQueryError)?;
    Ok(())
}

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/incomes", post(create_income))
        .route(
            "/incomes/:id",
            get(get_income).put(update_income).delete(delete_income),
        )
        .route("/expenses", post(create_expense))
        .route(
            "/expenses/:id",
            get(get_expense).put(update_expense).delete(delete_expense),
        )
        .route("/investments", post(create_investment))
        .route(
            "/investments/:id",
            get(get_investment)
                .put(update_investment)
                .delete(delete_investment),
        )
        .with_state(state)
}
