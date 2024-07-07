use axum::{
    body::Bytes,
    extract::{DefaultBodyLimit, Multipart},
    routing::post,
    Json, Router,
};

use crate::server::{AppResult, AppState};

use utoipa::{self, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(upload_transactions_file),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

#[derive(Debug, Default)]
enum Source {
    Firstrade,
    Ing,
    CreditAgricole,
    #[default]
    Custom,
}

impl Source {
    fn from_bytes(data: &[u8]) -> Self {
        match data {
            b"firstrade" => Self::Firstrade,
            b"ing" => Self::Ing,
            b"credit" => Self::CreditAgricole,
            _ => Self::Custom,
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
}

#[derive(Debug, Default)]
struct TransactionsFilesRequest {
    source: Source,
    files: Vec<TransactionFile>,
}

impl TransactionsFilesRequest {
    async fn new(multipart: &mut Multipart) -> Self {
        let mut request = Self::default();
        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap();
            let file_name = field.file_name().unwrap().to_owned();
            if name == "source" {
                let data = field.bytes().await.unwrap();
                request.source = Source::from_bytes(&data);
            } else {
                let data = field.bytes().await.unwrap();
                request.files.push(TransactionFile::new(file_name, data));
            };
        }
        request
    }

    async fn save(self)->Self{Self::default()}
}

#[utoipa::path(
    post,
    path = "upload/transactions",
    tag = "Transactions",
    request_body(content = Multipart, description = "A file with transactions", content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Created investment"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn upload_transactions_file(mut multipart: Multipart) -> AppResult<usize> {
    let _request = TransactionsFilesRequest::new(&mut multipart).await;
    Ok(Json(200))
}

const CONTENT_LENGTH_LIMIT: usize = 20 * 1024 * 1024;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/upload/transactions", post(upload_transactions_file))
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .with_state(state)
}
