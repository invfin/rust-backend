use crate::{
    db::schema::industries,
    server::{AppError, AppResult},
    AppState,
};

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use diesel::prelude::*;

use serde::{Deserialize, Serialize};
use utoipa::{self,OpenApi,ToSchema, ToResponse};

#[derive(OpenApi)]
#[openapi(
    paths(create_industry, delete_industry, read_industry, update_industry, list_industries),
    components(schemas(Industry), 
    responses(Industry)),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;


pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/industries", post(create_industry).get(list_industries))
        .route(
            "/industries/:id",
            get(read_industry).put(update_industry).delete(delete_industry),
        ).with_state(state)
}


#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Selectable,ToSchema, ToResponse)]
#[diesel(table_name = industries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Industry {
    id: i64,
    name: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    get, 
    path = "industries",
    responses(
            (status = 200, body = Vec<Industry>, description = "A paginated result of industries with key information"),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
        
)]
async fn list_industries(state: AppState) -> AppResult<Vec<Industry>> {
    let conn =  state.db_write().await?;
    let query = conn
        .interact(move |conn| {
            industries::table
                .select(Industry::as_select())
                .load::<Industry>(conn).map_err(AppError::DatabaseQueryError)
                
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??;

        Ok(Json(query))
}

#[utoipa::path(
    post,
    path = "industries",
    request_body = Industry,
    responses(
        (status = 200, body = Industry, description = "Create a new industry"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn create_industry(
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
    path = "industries/{id}",
    params(
        ("id" = i64, Path, description = "Industry ID")
    ),
    responses(
        (status = 200, body = Industry, description = "Read an industry by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn read_industry(Path(id): Path<i64>, state: AppState) -> AppResult<Industry> {
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
    path = "industries/{id}",
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
async fn update_industry(
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
    path = "industries/{id}",
    params(
        ("id" = i64, Path, description = "Industry ID")
    ),
    responses(
        (status = 200, description = "Delete an industry by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn delete_industry(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
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