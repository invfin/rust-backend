use diesel::{
query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods,  OptionalExtension, Queryable, RunQueryDsl, Selectable, SelectableHelper
};
use serde::{Deserialize, Serialize};
use axum::{ extract::{Path, Query}, routing::get, Json, Router};

use chrono::NaiveDate;
use crate::{db::{schema::companies, Paginate, Paginated}, server::{ AppError, AppJson, AppResult, AppState}};

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa::{ToSchema, ToResponse, IntoParams};
use utoipa;


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


#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct ShortCompanyQuery {
    pub country: Option<String>,
    pub exchange: Option<String>,
    pub industry: Option<String>,
    pub sector: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}


#[derive(Queryable, Debug, Serialize, Deserialize, Selectable, ToResponse, ToSchema)]
#[diesel(table_name = companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortCompany {
    pub ticker: String,
    pub name: Option<String>,
    pub country_id: Option<i64>,
    pub exchange_id: Option<i64>,
    pub industry_id: Option<i64>,
    pub sector_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
pub struct ShortCompanyResponse {
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
pub async fn get_short_companies(Query(query_params): Query<ShortCompanyQuery>, state: AppState) -> AppResult<ShortCompanyResponse> {
    let conn =  state.db_write().await?;
    let (query, total_pages) = conn
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
pub struct Company {
    pub ticker: String,
    pub name: Option<String>,
    pub website: Option<String>,
    pub state: Option<String>,
    pub ceo: Option<String>,
    pub image: Option<String>,
    pub city: Option<String>,
    pub employees: Option<String>,
    pub address: Option<String>,
    pub zip_code: Option<String>,
    pub cik: Option<String>,
    pub cusip: Option<String>,
    pub isin: Option<String>,
    pub description: Option<String>,
    pub ipo_date: Option<NaiveDate>,
    pub country_id: Option<i64>,
    pub exchange_id: Option<i64>,
    pub industry_id: Option<i64>,
    pub sector_id: Option<i64>,
    pub is_adr: bool,
    pub is_fund: bool,

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
pub async fn get_company(Path(ticker): Path<String>, state: AppState) -> AppResult<Company> {
    let conn =  state.db_write().await?;
     Ok(Json(conn
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

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/companies", get(get_short_companies))
        .route("/companies/:ticker",
            get(get_company)
        ).with_state(state)
}
