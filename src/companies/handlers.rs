use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods,  OptionalExtension, Queryable, RunQueryDsl, Selectable, SelectableHelper
    };
use serde::{Deserialize, Serialize};
use axum::{ extract::{Path, Query}, routing::get, Json, Router};

use chrono::NaiveDate;
use crate::{db::{schema::companies, Paginate}, server::{ AppError,  AppResult, AppState}};

use utoipa::{ToSchema, OpenApi, ToResponse, IntoParams, self};


#[derive(OpenApi)]
#[openapi(
    paths(get_short_companies,get_company),
    components(schemas(ShortCompanyResponse, ShortCompany, Company), responses(ShortCompanyResponse, ShortCompany, Company)),
    tags(
        (name = "Companies", description = "All about companies")
    ),
    security(
        ("token_jwt" = [])
    )
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/companies", get(get_short_companies))
        .route("/companies/:ticker",
            get(get_company)
        ).with_state(state)
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
struct ShortCompanyQuery {
    country: Option<String>,
    exchange: Option<String>,
    industry: Option<String>,
    sector: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
}


#[derive(Queryable, Debug, Serialize, Deserialize, Selectable, ToResponse, ToSchema)]
#[diesel(table_name = companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct ShortCompany {
    ticker: String,
    name: Option<String>,
    country_id: Option<i64>,
    exchange_id: Option<i64>,
    industry_id: Option<i64>,
    sector_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
struct ShortCompanyResponse {
    data: Vec<ShortCompany>,
    total_pages: i64,
}

#[utoipa::path(
    get, 
    path = "companies",
    params(ShortCompanyQuery),
    responses(
            (status = 200, body = ShortCompanyResponse, description = "A paginated result of companies with key information"),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
        
)]
async fn get_short_companies(Query(query_params): Query<ShortCompanyQuery>, state: AppState) -> AppResult<ShortCompanyResponse> {
    let (query, total_pages) = state.db_write().await?
        .interact(move |conn| {
            companies::table
                .select(ShortCompany::as_select())
                .paginate(query_params.page.unwrap_or(1))
                .per_page(query_params.per_page.unwrap_or(25))
                .load_and_count_pages::<ShortCompany>(conn).map_err(AppError::DatabaseQueryError)
                
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??;

        Ok(Json(ShortCompanyResponse{data:query, total_pages}))
}



#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, ToResponse, ToSchema)]
#[diesel(table_name = companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Company {
    ticker: String,
    name: Option<String>,
    website: Option<String>,
    state: Option<String>,
    ceo: Option<String>,
    image: Option<String>,
    city: Option<String>,
    employees: Option<String>,
    address: Option<String>,
    zip_code: Option<String>,
    cik: Option<String>,
    cusip: Option<String>,
    isin: Option<String>,
    description: Option<String>,
    ipo_date: Option<NaiveDate>,
    country_id: Option<i64>,
    exchange_id: Option<i64>,
    industry_id: Option<i64>,
    sector_id: Option<i64>,
    is_adr: bool,
    is_fund: bool,

}

#[utoipa::path(
    get, 
    path = "companies/{ticker}",
    params(("ticker", description = "Company's ticker")),
    responses(
            (status = 200, body = Company, description = "The company's full information"),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
)]
async fn get_company(Path(ticker): Path<String>, state: AppState) -> AppResult<Company> {
        Ok(Json(state.db_write().await?
        .interact(move |conn| {
            companies::table
                .filter(companies::ticker.eq(ticker))
                .select(Company::as_select())
                .first::<Company>(conn)
                .optional()
                .map_err(AppError::DatabaseQueryError)
                
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??.ok_or(AppError::DoesNotExist)?))
}
