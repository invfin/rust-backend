use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart},
    routing::post,
    Json, Router,
};
use diesel::Insertable;
use futures_util::{stream::FuturesUnordered, StreamExt};
use tokio::io::AsyncWriteExt;
use utoipa::{self, OpenApi};

use crate::{
    db::schema::transactions_files,
    server::{AppError, AppResult},
    AppState,
};

use super::files_parsers::{parse_credit_agricole, parse_firstrade, TransactionWrapper};

const CONTENT_LENGTH_LIMIT: usize = 20 * 1024 * 1024;

#[derive(OpenApi)]
#[openapi(
    paths(upload_transactions_file),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/upload/transactions", post(upload_transactions_file))
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
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
    account: String,
    account_id: Option<i64>,
    files: Vec<TransactionFile>,
}

#[derive(Insertable)]
#[diesel(table_name = transactions_files)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct TransactionsFile {
    user_id: i64,
    name: String,
    path: String,
}

impl TransactionsFile {
    fn new(user_id: i64, file: &TransactionFile) -> Self {
        Self {
            user_id,
            name: file.name.clone(),
            path: file.path(),
        }
    }
}

impl TransactionsFilesRequest {
    async fn new(multipart: &mut Multipart) -> Self {
        let mut request = Self::default();
        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap();
            let file_name = field.file_name().unwrap().to_owned();
            match name {
                "account_id" => {
                    let data = match field.bytes().await {
                        Ok(v) => String::from_utf8(v.to_vec())
                            .unwrap()
                            .parse::<i64>()
                            .ok(),
                        Err(_e) => None,
                    };

                    request.account_id = data;
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
