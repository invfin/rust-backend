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
            create_expense, get_expense, update_expense, delete_expense,
            create_investment, get_investment, update_investment, delete_investment
        ),
        components(schemas(Income, Expense, Investment, IncomeResponse, ExpenseResponse, InvestmentResponse), 
    responses(IncomeResponse, ExpenseResponse, InvestmentResponse)),
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

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Expense {
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
}

#[derive(Debug, Serialize, Deserialize, Queryable, ToResponse,Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = expenses)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct ExpenseResponse {
    id: i64,
    user_id: i64,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    amount: BigDecimal,
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

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = investments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Investment {
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    fee: BigDecimal,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    quantity: BigDecimal,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    cost: BigDecimal,
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
    asset_type: String,
    asset_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Queryable, ToResponse,Insertable, AsChangeset, ToSchema)]
#[diesel(table_name = investments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct InvestmentResponse {
    id: i64,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    fee: BigDecimal,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    quantity: BigDecimal,
    #[schema(value_type = BigDecimal, example = BigDecimal::default)]
    cost: BigDecimal,
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
    asset_type: String,
    asset_id: i64,
    #[schema(value_type = NaiveDateTime, example = NaiveDateTime::default)]
    created_at: NaiveDateTime,
    #[schema(value_type = NaiveDateTime, example = NaiveDateTime::default)]
    updated_at: NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "investments",
    tag = "Investments",
    request_body = Investment,
    responses(
        (status = 201, body = Income, description = "Created investment"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn create_investment(
    state: AppState,
    Json(new_income): Json<Investment>,
) -> AppResult<InvestmentResponse> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(investments::table)
                .values(&new_income)
                .get_result::<InvestmentResponse>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "investments/{id}",
    tag = "Investments",
    responses(
        (status = 200, body = InvestmentResponse, description = "The investment record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn get_investment(Path(id): Path<i64>, state: AppState) -> AppResult<InvestmentResponse> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            investments::table
                .find(id)
                .first::<InvestmentResponse>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(result))
}
#[utoipa::path(
    put,
    path = "investments/{id}",
    tag = "Investments",
    request_body = Investment,
    responses(
        (status = 200, body = InvestmentResponse, description = "Updated income record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]

async fn update_investment(
    Path(id): Path<i64>,
    state: AppState,
    Json(updated_income): Json<Investment>,
) -> AppResult<InvestmentResponse> {
    let conn = state.db_write().await?;

    let query = conn
        .interact(move |conn| {
            diesel::update(investments::table.find(id))
                .set(&updated_income)
                .get_result::<InvestmentResponse>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(query))
}

#[utoipa::path(
    delete,
    path = "investments/{id}",
    tag = "Investments",
    responses(
        (status = 204, body = usize, description = "Deleted investments record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn delete_investment(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::delete(investments::table.find(id))
                .execute(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "expenses/{id}",
    tag = "Expenses",
    responses(
        (status = 204, body = usize, description = "Deleted expenses record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn delete_expense(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::delete(expenses::table.find(id))
                .execute(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "expenses",
    tag = "Expenses",
    request_body = Income,
    responses(
        (status = 201, body = IncomeResponse, description = "Created expense"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn create_expense(
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
    get,
    path = "expenses/{id}",
    tag = "Expenses",
    responses(
        (status = 200, body = ExpenseResponse, description = "The expense record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    ),
    params(
        ("id" = u64, Path, description = "Expense database id to get Expense for"),
    )
)]
async fn get_expense(Path(id): Path<i64>, state: AppState) -> AppResult<ExpenseResponse> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            expenses::table
                .find(id)
                .first::<ExpenseResponse>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "expenses/{id}",
    tag = "Expenses",
    request_body = Expense,
    responses(
        (status = 200, body = ExpenseResponse, description = "Updated expense record"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn update_expense(
    Path(id): Path<i64>,
    state: AppState,
    Json(updated_income): Json<Expense>,
) -> AppResult<ExpenseResponse> {
    let conn = state.db_write().await?;

    let query = conn
        .interact(move |conn| {
            diesel::update(expenses::table.find(id))
                .set(&updated_income)
                .get_result::<ExpenseResponse>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(query))
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
