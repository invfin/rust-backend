use crate::{AppError, AppResult, AppState};
use crate::{AppError, AppResult, AppState};
use axum::{extract::Path, Json};
use axum::{extract::Path, Json};
use diesel::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = user_permissions]
pub struct UserPermission {
    pub id: i64,
    pub user_id: i64,
    pub permission: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/user_permissions",
    request_body = UserPermission,
    responses(
        (status = 200, body = UserPermission, description = "Create a new user permission"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_user_permission(
    state: AppState,Json(user_permission): Json<UserPermission>,
    
) -> AppResult<UserPermission> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(user_permissions::table)
                .values(&user_permission)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/user_permissions/{id}",
    params(
        ("id" = i64, Path, description = "UserPermission ID")
    ),
    responses(
        (status = 200, body = UserPermission, description = "Read a user permission by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_user_permission(
    Path(id): Path<i64>,
    state: AppState,
) -> AppResult<UserPermission> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            user_permissions::table
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
    path = "/user_permissions/{id}",
    params(
        ("id" = i64, Path, description = "UserPermission ID")
    ),
    request_body = UserPermission,
    responses(
        (status = 200, body = UserPermission, description = "Update a user permission by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_user_permission(
    Path(id): Path<i64>,
    state: AppState,Json(user_permission): Json<UserPermission>,
    
) -> AppResult<UserPermission> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(user_permissions::table.find(id))
                .set(&user_permission)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/user_permissions/{id}",
    params(
        ("id" = i64, Path, description = "UserPermission ID")
    ),
    responses(
        (status = 200, description = "Delete a user permission by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn delete_user_permission(Path(id): Path<i64>, state: AppState) -> AppResult<()> {
    let conn = state.db_write().await?;
    conn.interact(move |conn| {
        diesel::delete(user_permissions::table.find(id))
            .execute(conn)
            .map_err(AppError::DatabaseQueryError)
    })
    .await
    .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(())
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = permission_groups]
pub struct PermissionGroup {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/permission_groups",
    request_body = PermissionGroup,
    responses(
        (status = 200, body = PermissionGroup, description = "Create a new permission group"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_permission_group(
    state: AppState,Json(permission_group): Json<PermissionGroup>,
    
) -> AppResult<PermissionGroup> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(permission_groups::table)
                .values(&permission_group)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/permission_groups/{id}",
    params(
        ("id" = i64, Path, description = "PermissionGroup ID")
    ),
    responses(
        (status = 200, body = PermissionGroup, description = "Read a permission group by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_permission_group(
    Path(id): Path<i64>,
    state: AppState,
) -> AppResult<PermissionGroup> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            permission_groups::table
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
    path = "/permission_groups/{id}",
    params(
        ("id" = i64, Path, description = "PermissionGroup ID")
    ),
    request_body = PermissionGroup,
    responses(
        (status = 200, body = PermissionGroup, description = "Update a permission group by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_permission_group(
    Path(id): Path<i64>,
    state: AppState,Json(permission_group): Json<PermissionGroup>,
    
) -> AppResult<PermissionGroup> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(permission_groups::table.find(id))
                .set(&permission_group)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/permission_groups/{id}",
    params(
        ("id" = i64, Path, description = "PermissionGroup ID")
    ),
    responses(
        (status = 200, description = "Delete a permission group by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn delete_permission_group(Path(id): Path<i64>, state: AppState) -> AppResult<()> {
    let conn = state.db_write().await?;
    conn.interact(move |conn| {
        diesel::delete(permission_groups::table.find(id))
            .execute(conn)
            .map_err(AppError::DatabaseQueryError)
    })
    .await
    .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(())
}
