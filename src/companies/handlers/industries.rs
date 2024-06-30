use crate::{
    db::schema::industries,
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
            create_industry, delete_industry, read_industry, update_industry
        ),
        components(schemas(Industry), 
    responses(Industry)),
    security(
        ("token_jwt" = [])
    )
    )]
pub struct ApiDoc;


#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, ToSchema, ToResponse)]
#[diesel(table_name = industries)]
pub struct Industry {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/industries",
    request_body = Industry,
    responses(
        (status = 200, body = Industry, description = "Create a new industry"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_industry(
    state: AppState,Json(industry): Json<Industry>,
    
) -> AppResult<Industry> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(industries::table)
                .values(&industry)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/industries/{id}",
    params(
        ("id" = i64, Path, description = "Industry ID")
    ),
    responses(
        (status = 200, body = Industry, description = "Read an industry by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_industry(Path(id): Path<i64>, state: AppState) -> AppResult<Industry> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            industries::table
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
    path = "/industries/{id}",
    params(
        ("id" = i64, Path, description = "Industry ID")
    ),
    request_body = Industry,
    responses(
        (status = 200, body = Industry, description = "Update an industry by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_industry(
    Path(id): Path<i64>,
    state: AppState,Json(industry): Json<Industry>,
    
) -> AppResult<Industry> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(industries::table.find(id))
                .set(&industry)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/industries/{id}",
    params(
        ("id" = i64, Path, description = "Industry ID")
    ),
    responses(
        (status = 200, description = "Delete an industry by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn delete_industry(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
    let conn = state.db_write().await?;

    Ok(Json(
        conn.interact(move |conn| {
            diesel::delete(industries::table.find(id))
                .execute(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}