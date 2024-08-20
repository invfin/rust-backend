use axum::{routing::post, Json, Router};
use bigdecimal::BigDecimal;
use diesel::{Insertable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use utoipa::{self, OpenApi, ToSchema};

use crate::{
    db::schema::{accounts, fees},
    server::{AppError, AppResult},
    AppState,
};

#[derive(OpenApi)]
#[openapi(
    paths(create_account),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/accounts", post(create_account))
        .with_state(state)
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = fees)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Fee {
    recurrence: String,
    amount: BigDecimal, //TODO: use f64 and save it as bigD
    description: String,
    account_id: i64,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Account {
    name: String,
    category: String,
    subcategory: String,
    description: String,
}

#[derive(Debug, ToSchema, Serialize, Deserialize)]
struct AccountRequest {
    account: Account,
    fees: Vec<Fee>,
}

#[utoipa::path(
    post,
    path = "accounts",
    request_body = AccountRequest,
    responses(
        (status = 200, body = i64, description = "Create a new account"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn create_account(state: AppState, Json(req): Json<AccountRequest>) -> AppResult<i64> {
    Ok(Json(
        state
            .db_write()
            .await?
            .interact(move |conn| {
                let account_id = diesel::insert_into(accounts::table)
                    .values(req.account)
                    .returning(accounts::id)
                    .get_result(conn)
                    .map_err(AppError::DatabaseQueryError);
                if account_id.is_ok() && !req.fees.is_empty() {
                    let pk = account_id.as_ref().unwrap();
                    let mut fees = req.fees;
                    fees.iter_mut().for_each(|f| f.account_id = *pk);
                    diesel::insert_into(fees::table)
                        .values(fees)
                        .execute(conn)
                        .map_err(AppError::DatabaseQueryError)?;
                }
                account_id
            })
            .await
            .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}
