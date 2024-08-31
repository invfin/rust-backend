use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart},
    routing::post,
    Extension, Json, Router,
};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use futures_util::{stream::FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use utoipa::{self, OpenApi, ToSchema};

use crate::{
    db::schema::{assets_details, investment_details, transactions, transactions_details},
    server::{AppError, AppResult, JWTUserRequest},
    AppState,
};

use super::files_parsers::{parse_credit_agricole, parse_firstrade, TransactionWrapper};

const CONTENT_LENGTH_LIMIT: usize = 20 * 1024 * 1024;

#[derive(OpenApi)]
#[openapi(
    paths(upload_transactions_file, create_transaction),
    components(schemas(AssetDetail, InvestmentDetail, TransactionDetail, Transaction, TransactionRequest)),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/upload/transactions", post(upload_transactions_file))
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .route("/transactions", post(create_transaction))
        .with_state(state)
}

#[derive(Debug, Default)]
enum Source {
    Firstrade,
    CreditAgricole,
    #[default]
    Custom,
}

impl Source {
    fn from_bytes(data: &[u8]) -> Self {
        match data {
            b"firstrade" => Self::Firstrade,
            b"credit" => Self::CreditAgricole,
            _ => Self::Custom,
        }
    }

    async fn parse_file(&self, user_id: i64, file: &TransactionFile) -> Vec<TransactionWrapper> {
        file.save().await;
        let path = file.path();
        match self {
            Self::Firstrade => parse_firstrade(user_id, &path).await,
            Self::CreditAgricole => parse_credit_agricole(user_id, &path).await,
            Self::Custom => todo!(),
        }
    }
}

#[derive(Debug)]
struct TransactionFile {
    name: String,
    data: Bytes,
}

impl TransactionFile {
    fn new(name: String, data: Bytes) -> Self {
        Self { name, data }
    }

    fn path(&self) -> String {
        format!("media/{}", &self.name)
    }

    async fn save(&self) {
        tokio::fs::File::create(&self.path())
            .await
            .unwrap()
            .write_all(&self.data)
            .await
            .unwrap();
    }
}

#[derive(Debug, Default)]
struct TransactionsFilesRequest {
    user_id: i64,
    source: Source,
    account_id: Option<i64>,
    currency_id: Option<i64>,
    files: Vec<TransactionFile>,
}

impl TransactionsFilesRequest {
    async fn new(multipart: &mut Multipart) -> Self {
        let mut request = Self::default();
        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap();
            let file_name = field.file_name().unwrap().to_owned();
            match name {
                "source" => {
                    let data = field.bytes().await.unwrap();
                    request.source = Source::from_bytes(&data);
                }
                "currencyId" => {
                    request.currency_id = match field.bytes().await {
                        Ok(v) => String::from_utf8(v.to_vec()).unwrap().parse::<i64>().ok(),
                        Err(_e) => None,
                    };
                }
                "account_id" => {
                    request.account_id = match field.bytes().await {
                        Ok(v) => String::from_utf8(v.to_vec()).unwrap().parse::<i64>().ok(),
                        Err(_e) => None,
                    };
                }
                _ => {
                    let data = field.bytes().await.unwrap();
                    request.files.push(TransactionFile::new(file_name, data));
                }
            }
        }
        request
    }

    async fn process_files(&self, file: &TransactionFile) {
        //TODO: for now we assume that we have always the exchange rates for USD/EUR
        let _transactions = self.source.parse_file(self.user_id, file).await;
    }

    async fn save(self) -> AppResult<usize> {
        let tasks: FuturesUnordered<_> = FuturesUnordered::new();

        for file in &self.files {
            tasks.push(self.process_files(file));
        }
        let _: Vec<()> = tasks.collect().await;
        Ok(Json(200))
    }
}

#[utoipa::path(
    post,
    path = "upload/transactions",
    tag = "Transactions",
    request_body(content = Multipart, description = "A file with transactions", content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Created transactions"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn upload_transactions_file(state: AppState, mut multipart: Multipart) -> AppResult<usize> {
    let conn = state.db_write().await?;
    let request = TransactionsFilesRequest::new(&mut multipart).await;

    let new_files = request.files.iter().map(|f| {});
    conn.interact(move |conn| {})
        .await
        .map_err(AppError::DatabaseConnectionInteractError)?;
    request.save().await
}

#[derive(Debug, Deserialize, Serialize, Insertable, ToSchema)]
#[diesel(table_name = assets_details)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct AssetDetail {
    category: String,
    name: String,
    company_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Insertable, ToSchema)]
#[diesel(table_name = investment_details)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct InvestmentDetail {
    quantity: f64,
    cost: BigDecimal,
    #[serde(skip)]
    asset_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Insertable, ToSchema)]
#[diesel(table_name = transactions_details)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct TransactionDetail {
    description: Option<String>,
    comment: Option<String>,
    original_amount: BigDecimal,
    fee: BigDecimal,
}

#[derive(Debug, Deserialize, Serialize, Insertable, ToSchema)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Transaction {
    #[serde(skip)]
    user_id: i64,
    #[serde(skip)]
    details_id: Option<i64>, //TODO: use option instead of skip?
    account_id: i64,
    date: chrono::NaiveDate,
    amount: BigDecimal,
    category: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
struct TransactionRequest {
    details: TransactionDetail,
    investment_details: Option<InvestmentDetail>,
    transaction: Transaction,
    asset: Option<AssetDetail>,
}

#[utoipa::path(
    post,
    path = "transactions",
    tag = "Transactions",
    request_body = TransactionRequest,
    responses(
        (status = 201, body = usize, description = "Created transactions"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn create_transaction(
    state: AppState,
    Extension(current_user): Extension<JWTUserRequest>,
    Json(req): Json<TransactionRequest>,
) -> AppResult<i64> {
    state
        .db_write()
        .await?
        .interact(move |conn| {
            conn.transaction(|conn| {
                let mut investment_details_id: Option<i64> = None;
                if req.investment_details.is_some() && req.asset.is_some() {
                    let asset = req.asset.unwrap();
                    let asset_id: i64 = diesel::insert_into(assets_details::table)
                        .values(asset)
                        .returning(assets_details::id)
                        .get_result(conn)
                        .map_err(AppError::DatabaseQueryError)?;

                    let mut investment = req.investment_details.unwrap();
                    investment.asset_id = asset_id;
                    investment_details_id = Some(
                        diesel::insert_into(investment_details::table)
                            .values(investment)
                            .returning(investment_details::id)
                            .get_result(conn)
                            .map_err(AppError::DatabaseQueryError)?,
                    );
                }

                let details_id: i64 = diesel::insert_into(transactions_details::table)
                    .values((
                        transactions_details::investment_details_id.eq(investment_details_id),
                        req.details,
                    ))
                    .returning(transactions_details::id)
                    .get_result(conn)
                    .map_err(AppError::DatabaseQueryError)?;

                let mut transaction = req.transaction;
                transaction.user_id = current_user.id;
                transaction.details_id = Some(details_id);

                let transaction_id = diesel::insert_into(transactions::table)
                    .values(transaction)
                    .returning(transactions::id)
                    .get_result(conn)
                    .map_err(AppError::DatabaseQueryError)?;

                Ok(Json(transaction_id))
            })
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)?
}
