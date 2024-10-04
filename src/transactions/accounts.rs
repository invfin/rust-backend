use axum::{routing::post, Extension, Json, Router};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::{self, OpenApi, ToSchema};

use crate::{
    db::schema::{accounts, fees, rates_return},
    server::{AppError, AppResult, JWTUserRequest},
    AppState,
};

#[derive(OpenApi)]
#[openapi(
    paths(create_account, create_rate, list_accounts),
    components(schemas(AccountRequest, Amount, Account, AccountCore)),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/accounts", post(create_account).get(list_accounts))
        .route("/rates", post(create_rate))
        .with_state(state)
}

#[derive(Debug, Insertable, Serialize, Queryable, Selectable, Deserialize)]
#[diesel(table_name = fees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Fee {
    description: Option<String>,
    active: bool,
    percentage: bool,
    account_id: i64,
    recurrence: String,
    amount: BigDecimal,
}

impl Fee {
    fn new(amount: &Amount, account_id: i64) -> Self {
        Self {
            account_id,
            active: true,
            percentage: amount.percentage,
            recurrence: amount.recurrence.clone(),
            amount: amount.amount.clone(),
            description: amount.description.clone(),
        }
    }
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = rates_return)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Rate {
    description: Option<String>,
    active: bool,
    percentage: bool,
    account_id: i64,
    recurrence: String,
    amount: BigDecimal,
}

impl Rate {
    fn new(amount: &Amount, account_id: i64) -> Self {
        Self {
            account_id,
            active: true,
            percentage: amount.percentage,
            recurrence: amount.recurrence.clone(),
            amount: amount.amount.clone(),
            description: amount.description.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
struct Amount {
    description: Option<String>,
    percentage: bool,
    recurrence: String,
    amount: BigDecimal,
}

#[derive(Debug, Selectable, Queryable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct AccountCore {
    pub id: i64,
    name: String,
    company: String,
    currency_id: i64,
}

#[derive(Debug, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
struct Account {
    name: String,
    category: String,
    company: String,
    description: Option<String>,
    currency_id: i64,
    amount: BigDecimal,
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct AccountRequest {
    #[serde(flatten)]
    account: Account,
    fees: Vec<Amount>,
    rates: Vec<Amount>,
}

pub fn create_account_from_request(
    req: AccountRequest,
    current_user_id: i64,
    conn: &mut PgConnection,
) -> Result<AccountCore, AppError> {
    let account = diesel::insert_into(accounts::table)
        .values((accounts::user_id.eq(current_user_id), req.account))
        .returning(AccountCore::as_returning())
        .get_result(conn)
        .map_err(AppError::DatabaseQueryError)?;
    let pk = account.id;
    if !req.fees.is_empty() {
        let fees = req
            .fees
            .iter()
            .map(|f| Fee::new(f, pk))
            .collect::<Vec<Fee>>();
        diesel::insert_into(fees::table)
            .values(fees)
            .execute(conn)
            .map_err(AppError::DatabaseQueryError)?;
    }
    if !req.rates.is_empty() {
        let rates = req
            .rates
            .iter()
            .map(|f| Rate::new(f, pk))
            .collect::<Vec<Rate>>();
        diesel::insert_into(rates_return::table)
            .values(rates)
            .execute(conn)
            .map_err(AppError::DatabaseQueryError)?;
    }
    Ok(account)
}

#[utoipa::path(
    get,
    path = "accounts",
    responses(
        (status = 200, body = Vec<AccountCore>, description = "Get all the accounts"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn list_accounts(
    state: AppState,
    Extension(current_user): Extension<JWTUserRequest>,
) -> AppResult<Vec<AccountCore>> {
    state
        .db_write()
        .await?
        .interact(move |conn| {
            accounts::table
                .filter(accounts::user_id.eq(current_user.id))
                .select(AccountCore::as_select())
                .load(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)?
        .map(Json)
}

#[utoipa::path(
    post,
    path = "accounts",
    request_body = AccountRequest,
    responses(
        (status = 200, body = AccountCore, description = "Create a new account"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn create_account(
    state: AppState,
    Extension(current_user): Extension<JWTUserRequest>,
    Json(req): Json<AccountRequest>,
) -> AppResult<AccountCore> {
    state
        .db_write()
        .await?
        .interact(move |conn| {
            conn.transaction(|conn| create_account_from_request(req, current_user.id, conn))
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)?
        .map(Json)
}

#[utoipa::path(
    post,
    path = "rates",
    request_body = Fee,
    responses(
        (status = 200, body = Fee, description = "Add a new rate or fee to an account"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn create_rate(state: AppState, Json(rate): Json<Fee>) -> AppResult<Fee> {
    Ok(Json(
        state
            .db_write()
            .await?
            .interact(move |conn| {
                diesel::insert_into(fees::table)
                    .values(&rate)
                    .returning(Fee::as_returning())
                    .get_result(conn)
                    .map_err(AppError::DatabaseQueryError)
            })
            .await
            .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}
