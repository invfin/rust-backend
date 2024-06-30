use crate::{
    db::schema::periods,
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
            create_period, delete_period, read_period, update_period
        ),
        components(schemas(Period), 
    responses(Period)),
    security(
        ("token_jwt" = [])
    )
    )]
pub struct ApiDoc;


#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, ToSchema, ToResponse)]
#[diesel(table_name = periods)]
pub struct Period {
    pub id: i64,
    pub year: i32,
    pub period: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    post,
    path = "/periods",
    request_body = Period,
    responses(
        (status = 200, body = Period, description = "Create a new period"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn create_period(state: AppState, Json(period): Json<Period>) -> AppResult<Period> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::insert_into(periods::table)
                .values(&period)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    get,
    path = "/periods/{id}",
    params(
        ("id" = i64, Path, description = "Period ID")
    ),
    responses(
        (status = 200, body = Period, description = "Read a period by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn read_period(Path(id): Path<i64>, state: AppState) -> AppResult<Period> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            periods::table
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
    path = "/periods/{id}",
    params(
        ("id" = i64, Path, description = "Period ID")
    ),
    request_body = Period,
    responses(
        (status = 200, body = Period, description = "Update a period by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn update_period(
    Path(id): Path<i64>,
    state: AppState,Json(period): Json<Period>,
    
) -> AppResult<Period> {
    let conn = state.db_write().await?;
    let result = conn
        .interact(move |conn| {
            diesel::update(periods::table.find(id))
                .set(&period)
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/periods/{id}",
    params(
        ("id" = i64, Path, description = "Period ID")
    ),
    responses(
        (status = 200, description = "Delete a period by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
pub async fn delete_period(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
    let conn = state.db_write().await?;

    Ok(Json(
        conn.interact(move |conn| {
            diesel::delete(periods::table.find(id))
                .execute(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}