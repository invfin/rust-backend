use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use diesel::{
    associations::{Associations, Identifiable},
    query_dsl::methods::{FilterDsl, SelectDsl},
    BelongingToDsl, ExpressionMethods, OptionalExtension, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::{
        schema::{definitions, definitions_content},
        Paginate,
    },
    server::{AppError, AppResult, AppState},
};
use chrono::NaiveDateTime;

use utoipa;

use utoipa::{IntoParams, OpenApi, ToResponse, ToSchema};

#[derive(Serialize, Deserialize, Debug)]
enum Status {
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

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/diccionario-financiero", get(list_definitions))
        .route("/definicion/:slug", get(get_definition))
        .with_state(state)
}

#[derive(Queryable, Debug, Serialize, Deserialize, Selectable, ToResponse, ToSchema)]
#[diesel(table_name = definitions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct ShortDefinition {
    title: String,
    resume: String,
    slug: String,
    total_views: i64,
    updated_at: NaiveDateTime,
    total_votes: i64,
    author_id: i64,
    thumbnail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, IntoParams)]
struct ShortDefinitionQuery {
    category: Option<String>,
    author: Option<String>,
    publication_date: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
struct ShortDefinitionResponse {
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
async fn list_definitions(
    Query(query_params): Query<ShortDefinitionQuery>,
    state: AppState,
) -> AppResult<ShortDefinitionResponse> {
    let (data, total_pages) = state
        .db_write()
        .await?
        .interact(move |conn| {
            definitions::table
                .select(ShortDefinition::as_select())
                .paginate(query_params.page.unwrap_or(1))
                .per_page(query_params.per_page.unwrap_or(25))
                .load_and_count_pages::<ShortDefinition>(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;

    Ok(Json(ShortDefinitionResponse { data, total_pages }))
}

#[derive(Serialize, Deserialize, ToResponse, Debug, ToSchema)]
struct DefinitionResponse {
    definition: Definition,
    content: Vec<DefinitionContent>,
}

#[derive(
    Serialize,
    Deserialize,
    ToResponse,
    Queryable,
    Selectable,
    Identifiable,
    Debug,
    PartialEq,
    ToSchema,
)]
#[diesel(table_name = definitions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Definition {
    id: i64,
    title: String,
    resume: String,
    slug: String,
    created_at: NaiveDateTime,
    total_views: i64,
    updated_at: NaiveDateTime,
    total_votes: i64,
    author_id: i64,
    thumbnail: Option<String>,
}

#[derive(
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    ToResponse,
    Selectable,
    Associations,
    Debug,
    PartialEq,
    ToSchema,
)]
#[diesel(belongs_to(Definition))]
#[diesel(table_name = definitions_content)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct DefinitionContent {
    id: i64,
    title: String,
    slug: String,
    content: String,
    order: i64,
    definition_id: i64,
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
async fn get_definition(
    Path(slug): Path<String>,
    state: AppState,
) -> AppResult<DefinitionResponse> {
    Ok(Json(
        state
            .db_write()
            .await?
            .interact(move |conn| {
                let definition = definitions::table
                    .filter(definitions::slug.eq(slug))
                    .select(Definition::as_select())
                    .first::<Definition>(conn)
                    .optional()
                    .map_err(AppError::DatabaseQueryError)
                    .unwrap();

                match definition {
                    Some(v) => {
                        let content = DefinitionContent::belonging_to(&v)
                            .select(DefinitionContent::as_select())
                            .load::<DefinitionContent>(conn)
                            .map_err(AppError::DatabaseQueryError);

                        match content {
                            Ok(q) => Ok(DefinitionResponse {
                                definition: v,
                                content: q,
                            }),
                            Err(_e) => Err(AppError::DoesNotExist),
                        }
                    }
                    None => Err(AppError::DoesNotExist),
                }
            })
            .await??,
    ))
}
