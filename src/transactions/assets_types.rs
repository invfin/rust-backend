use crate::{AppError, AppResult, AppState};
use axum::{extract::Path, Json};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = asset_types]
pub struct AssetType {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "asset_types",
    request_body = AssetType,
    responses(
        (status = 200, body = AssetType, description = "Create a new asset type"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_asset_type(
    state: AppState,
    Json(asset_type): Json<AssetType>,
) -> AppResult<AssetType> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(asset_types::table)
                .values(&asset_type)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "asset_types/{id}",
    params(
        ("id" = i64, Path, description = "AssetType ID")
    ),
    responses(
        (status = 200, body = AssetType, description = "Read an asset type by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_asset_type(Path(id): Path<i64>, state: AppState) -> AppResult<AssetType> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            asset_types::table
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
    path = "asset_types/{id}",
    params(
        ("id" = i64, Path, description = "AssetType ID")
    ),
    request_body = AssetType,
    responses(
        (status = 200, body = AssetType, description = "Update an asset type by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_asset_type(
    Path(id): Path<i64>,
    state: AppState,
    Json(asset_type): Json<AssetType>,
) -> AppResult<AssetType> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(asset_types::table.find(id))
                .set(&asset_type)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "asset_types/{id}",
    params(
        ("id" = i64, Path, description = "AssetType ID")
    ),
    responses(
        (status = 200, description = "Delete an asset type by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn delete_asset_type(Path(id): Path<i64>, state: AppState) -> AppResult<()> {
    let conn = state.db_write().await?;
    conn.interact(move |conn| {
        diesel::delete(asset_types::table.find(id))
            .execute(conn)
            .map_err(AppError::DatabaseQueryError)
    })
    .await
    .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(())
}
