use crate::{
    db::schema::countries,
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
use utoipa::{self, OpenApi, ToResponse, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(create_country, delete_country, read_country, update_country, list_countries),
    components(schemas(Country),
    responses(Country)),
    security(("token_jwt" = []))
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Countries
        .route("/countries", post(create_country).get(list_countries))
        .route(
            "/countries/:id",
            get(read_country).put(update_country).delete(delete_country),
        )
        .with_state(state)
}

#[derive(
    Queryable, Insertable, AsChangeset, Serialize, Selectable, Deserialize, ToResponse, ToSchema,
)]
#[diesel(table_name = countries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Country {
    id: i64,
    name: String,
    iso: String,
    alpha_2_code: String,
    alpha_3_code: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

#[utoipa::path(
    get,
    path = "countries",
    responses(
            (status = 200, body = Vec<Country>, description = "A paginated result of countries with key information"),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
)]
async fn list_countries(state: AppState) -> AppResult<Vec<Country>> {
    Ok(Json(
        state
            .db_write()
            .await?
            .interact(move |conn| {
                countries::table
                    .select(Country::as_select())
                    .load::<Country>(conn)
                    .map_err(AppError::DatabaseQueryError)
            })
            .await
            .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}

#[utoipa::path(
    post,
    path = "countries",
    request_body = Country,
    responses(
        (status = 200, body = Country, description = "Create a new country"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn create_country(state: AppState, Json(country): Json<Country>) -> AppResult<Country> {
    Ok(Json(
        state
            .db_write()
            .await?
            .interact(move |conn| {
                diesel::insert_into(countries::table)
                    .values(&country)
                    .get_result(conn)
                    .map_err(AppError::DatabaseQueryError)
            })
            .await
            .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}

#[utoipa::path(
    get,
    path = "countries/{id}",
    params(
        ("id" = i64, Path, description = "Country ID")
    ),
    responses(
        (status = 200, body = Country, description = "Read a country by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn read_country(Path(id): Path<i64>, state: AppState) -> AppResult<Country> {
    Ok(Json(
        state
            .db_write()
            .await?
            .interact(move |conn| {
                countries::table
                    .find(id)
                    .first(conn)
                    .map_err(AppError::DatabaseQueryError)
            })
            .await
            .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}

pub enum CountryIndexes {
    Id(i64),
    Iso(String),
}

pub fn get_country_id(idx: CountryIndexes, conn: &mut PgConnection) -> Result<i64, AppError> {
    match idx {
        CountryIndexes::Id(v) => countries::table
            .filter(countries::id.eq(v))
            .select(countries::id)
            .first(conn)
            .map_err(AppError::DatabaseQueryError),
        CountryIndexes::Iso(v) => countries::table
            .filter(countries::iso.eq(v))
            .select(countries::id)
            .first(conn)
            .map_err(AppError::DatabaseQueryError),
    }
}

#[utoipa::path(
    put,
    path = "countries/{id}",
    params(
        ("id" = i64, Path, description = "Country ID")
    ),
    request_body = Country,
    responses(
        (status = 200, body = Country, description = "Update a country by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn update_country(
    Path(id): Path<i64>,
    state: AppState,
    Json(country): Json<Country>,
) -> AppResult<Country> {
    Ok(Json(
        state
            .db_write()
            .await?
            .interact(move |conn| {
                diesel::update(countries::table.find(id))
                    .set(&country)
                    .get_result(conn)
                    .map_err(AppError::DatabaseQueryError)
            })
            .await
            .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}

#[utoipa::path(
    delete,
    path = "countries/{id}",
    params(("id" = i64, Path, description = "Country ID")),
    responses(
        (status = 200, body = usize, description = "Delete a country by ID"),
        (status = "4XX", body = ErrorMessage, description = "Client error"),
        (status = "5XX", body = ErrorMessage, description = "Server error"),
    )
)]
async fn delete_country(Path(id): Path<i64>, state: AppState) -> AppResult<usize> {
    Ok(Json(
        state
            .db_write()
            .await?
            .interact(move |conn| {
                diesel::delete(countries::table.find(id))
                    .execute(conn)
                    .map_err(AppError::DatabaseQueryError)
            })
            .await
            .map_err(AppError::DatabaseConnectionInteractError)??,
    ))
}
