use crate::{
    db::schema::sectors,
    server::{AppError, AppResult},
    AppState,
};
use axum::{extract::Path, Json};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::{self, ToResponse};
use utoipa::OpenApi;
use utoipa::ToSchema;


#[derive(OpenApi)]
#[openapi(
        paths(
            create_sector, delete_sector, read_sector, update_sector
        ),
        components(schemas(Sector), 
    responses(Sector)),
    security(
        ("token_jwt" = [])
    )
    )]
pub struct ApiDoc;


#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, ToSchema, ToResponse)]
#[diesel(table_name = sectors)]
pub struct Sector {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/sectors",
    request_body = Sector,
    responses(
        (status = 200, body = Sector, description = "Create a new sector"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_sector(state: AppState, Json(sector): Json<Sector>) -> AppResult<Sector> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(sectors::table)
                .values(&sector)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/sectors/{id}",
    params(
        ("id" = i64, Path, description = "Sector ID")
    ),
    responses(
        (status = 200, body = Sector, description = "Read a sector by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_sector(Path(id): Path<i64>, state: AppState) -> AppResult<Sector> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            sectors::table
                .find(id)
                .first(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    put,
    path = "/sectors/{id}",
    params(
        ("id" = i64, Path, description = "Sector ID")
    ),
    request_body = Sector,
    responses(
        (status = 200, body = Sector, description = "Update a sector by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_sector(
    Path(id): Path<i64>,
    state: AppState,
    Json(sector): Json<Sector>,
) -> AppResult<Sector> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(sectors::table.find(id))
                .set(&sector)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/sectors/{id}",
    params(
        ("id" = i64, Path, description = "Sector ID")
    ),
    responses(
        (status = 200, description = "Delete a sector by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]

pub async fn delete_sector(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
    let conn = state.db_write().await?;

    Ok(Json(
        conn.interact(move |conn| {
            diesel::delete(sectors::table.find(id))
                .execute(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}