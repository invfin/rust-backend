use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use diesel::{
    associations::{Associations, Identifiable}, query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, OptionalExtension, Queryable, RunQueryDsl, Selectable, SelectableHelper
};
use serde::{Deserialize, Serialize};

use crate::{
    db::{schema::{definitions, definitions_categories, definitions_content}, Paginate},
    server::{AppError,  AppResult, AppState},
};
use chrono::{DateTime, NaiveDateTime, Utc};

use utoipa;
use utoipa::{
     OpenApi,
};
use utoipa::{IntoParams, ToResponse, ToSchema};


#[derive(Serialize, Deserialize,Debug)]
pub enum Status {
    Publsihed,
    Draft,
    NeedRevision,
    Scheduled,
}

#[derive(OpenApi)]
#[openapi(
    paths(list_definitions, get_definition),
    components(schemas(ShortDefinitionResponse, ShortDefinition,  Definition, DefinitionContent), 
    responses(ShortDefinitionResponse, ShortDefinition, Definition, DefinitionContent)),
    tags(
        (name = "Definitions", description = "All about the dictionary")
    ),
    security(
        ("token_jwt" = [])
    )
)]
pub struct ApiDoc;


#[derive(Queryable, Debug, Serialize, Deserialize, Selectable, ToResponse, ToSchema)]
#[diesel(table_name = definitions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortDefinition {
    pub title: String,
    pub resume: String,
    pub slug: String,
    pub total_views: i64,
    pub updated_at: NaiveDateTime,
    pub total_votes: i64,
    pub author_id: i64,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
pub struct ShortDefinitionQuery {
    pub category: Option<String>,
    pub author: Option<String>,
    pub publication_date: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
pub struct ShortDefinitionResponse {
    data: Vec<ShortDefinition>,
    total_pages: i64,
}

#[utoipa::path(
    get, 
    path = "diccionario-financiero",
    params(ShortDefinitionQuery),
    responses(
            (status = 200, body = ShortDefinitionQuery, description = "A paginated result of definitions with key information"),
            (status = "4XX", body = ErrorMessage, description = "Oupsi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Oupsi daisy"),
        )
        
)]
pub async fn list_definitions(Query(query_params): Query<ShortDefinitionQuery>, state: AppState) -> AppResult<ShortDefinitionResponse> {
    let conn =  state.db_write().await?;
    let (query, total_pages) = conn
        .interact(move |conn| {
            definitions::table
                .select(ShortDefinition::as_select())
                .paginate(query_params.page.unwrap_or(1))
                .per_page(query_params.per_page.unwrap_or(25))
                .load_and_count_pages::<ShortDefinition>(conn).map_err(AppError::DatabaseQueryError)
                
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??;

        Ok(Json(ShortDefinitionResponse{data:query, total_pages}))
}

#[derive(Serialize, Deserialize, ToResponse, Debug, ToSchema)]
pub struct DefinitionResponse{
    pub definition: Definition,
    pub content: Vec<DefinitionContent>,
}

#[derive(Serialize, Deserialize, ToResponse,Queryable, Selectable, Identifiable,  Debug, PartialEq, ToSchema)]
#[diesel(table_name = definitions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Definition {
    pub id: i64,
    pub title: String,
    pub resume: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub total_views: i64,
    pub updated_at: NaiveDateTime,
    pub total_votes: i64,
    pub author_id: i64,
    pub thumbnail: Option<String>
}

#[derive(Serialize, Deserialize,Queryable, Identifiable, ToResponse, Selectable, Associations,Debug, PartialEq, ToSchema)]
#[diesel(belongs_to(Definition))]
#[diesel(table_name = definitions_content)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DefinitionContent {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub order: i64,
    pub definition_id: i64,
}

#[utoipa::path(
    get, 
    path = "definicion/{slug}",
    params(("slug", description = "Definition's slug")),
    responses(
            (status = 200, body = DefinitionResponse, description = "A definition of a given term"),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
        
)]
pub async fn get_definition(Path(slug): Path<String>, state: AppState) -> AppResult<DefinitionResponse> {
    let conn =  state.db_write().await?;
let result = conn
.interact(move |conn| {

    let definition = definitions::table
        .filter(definitions::slug.eq(slug))
        .select(Definition::as_select())
        .first::<Definition>(conn)
        .optional()
        .map_err(AppError::DatabaseQueryError).unwrap();

    match definition {
        Some(v) => {let content: Vec<DefinitionContent> = DefinitionContent::belonging_to(&v)
            .select(DefinitionContent::as_select())
            .load::<DefinitionContent>(conn)
            .map_err(AppError::DatabaseQueryError);
    
            Ok(DefinitionResponse {v, content})},
        None => Err(AppError::DoesNotExist)
    }   
})
.await.map_err(AppError::DatabaseConnectionInteractError).ok_or(AppError::DoesNotExist)?;

        Ok(Json(result))
}

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/diccionario-financiero", get(list_definitions))
        .route("/definicion/:slug",
            get(get_definition)
        ).with_state(state)
}
