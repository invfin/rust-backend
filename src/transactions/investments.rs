use axum::{
    extract::{DefaultBodyLimit, Multipart, Path},
    routing::{get, post},
    Json, Router,
};
use futures_util::stream::StreamExt;
use bigdecimal::BigDecimal;
use diesel::{
    AsChangeset, Insertable, QueryDsl, Queryable,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::{
        schema::investments,
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
          upload_investment,  create_investment, get_investment, update_investment, delete_investment
        ),
        components(schemas( Investment, InvestmentResponse), 
    responses( InvestmentResponse)),
    security(
        ("token_jwt" = [])
    )
    )]
pub struct ApiDoc;

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
    post,
    path = "upload/investments",
    tag = "Investments",
    request_body(content = Multipart, description = "A file with the investments", content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Created investment"),
        (status = "4XX", body = ErrorMessage, description = "Validation errors"),
        (status = "5XX", body = ErrorMessage, description = "Internal server error")
    )
)]
async fn upload_investment(
    state: AppState,
    mut multipart: Multipart
) -> AppResult<usize> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );
    };

    Ok(Json(200))
}

const CONTENT_LENGTH_LIMIT: usize = 20 * 1024 * 1024;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/upload/investments", post(upload_investment))
        .layer(DefaultBodyLimit::max(CONTENT_LENGTH_LIMIT))
        .route("/investments", post(create_investment))
        .route(
            "/investments/:id",
            get(get_investment)
                .put(update_investment)
                .delete(delete_investment),
        )
        .with_state(state)
}
