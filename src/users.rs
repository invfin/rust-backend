use axum::{routing::post, Json, Router};
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, Insertable, OptionalExtension, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};
use serde::{Deserialize, Serialize};

use crate::{
    db::schema::users,
    server::{create_token, AppError, AppJson, AppResult, AppState},
};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use utoipa;
use utoipa::OpenApi;
use utoipa::{ToResponse, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(login,register),
    components(
        schemas(LoginPayload, RegisterPayload, LoginResponse),
        responses(LoginResponse)
    ),
    tags(
        (name = "Users", description = "All about users")
    ),
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(state)
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    username: String,
    email: String,
    is_active: bool,
    is_superuser: bool,
    is_staff: bool,
    is_test: bool,
    password: String,
}

impl NewUser {
    fn new(query_params: RegisterPayload) -> Self {
        Self {
            username: query_params.username,
            email: query_params.email,
            is_active: true,
            is_superuser: false,
            is_staff: false,
            is_test: false,
            password: hash_password(&query_params.password),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
struct RegisterPayload {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct LoginUser {
    id: i64,
    username: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
struct LoginResponse {
    username: String,
    email: String,
    token: String,
}

impl LoginResponse {
    async fn new(user: &LoginUser, state: &AppState) -> AppResult<LoginResponse> {
        Ok(Json(Self {
            username: user.username.clone(),
            email: user.email.clone(),
            token: create_token(user.id, "admin".to_owned(), state).await?,
        }))
    }
}

#[utoipa::path(
    post,
    path = "login",
    request_body = LoginPayload,
    description = "User logedin",
    responses(
            (status = 200, body = LoginResponse),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
)]
async fn login(
    state: AppState,
    AppJson(payload): AppJson<LoginPayload>,
) -> AppResult<LoginResponse> {
    let conn = state.db_write().await?;
    let (user, password) = conn
        .interact(move |conn| {
            users::table
                .filter(users::email.eq(payload.email))
                .select(((users::id, users::username, users::email), users::password))
                .first::<(LoginUser, String)>(conn)
                .optional()
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??
        .ok_or(AppError::DoesNotExist)?;
    let parsed_hash = PasswordHash::new(&password).unwrap();
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(AppError::WrongPassword)?;
    LoginResponse::new(&user, &state).await
}

#[utoipa::path(
    post,
    path = "register",
    description = "Register user",
    request_body = RegisterPayload,
    responses(
            (status = 200, body = LoginResponse, description = "User created and registered"),
            (status = "4XX", body = ErrorMessage, description = "Opusi daisy"),
            (status = "5XX", body = ErrorMessage, description = "Opusi daisy"),
        )
)]
async fn register(
    state: AppState,
    AppJson(payload): AppJson<RegisterPayload>,
) -> AppResult<LoginResponse> {
    let user = state
        .db_write()
        .await?
        .interact(move |conn| {
            diesel::insert_into(users::table)
                .values(NewUser::new(payload))
                .returning(LoginUser::as_returning())
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    LoginResponse::new(&user, &state).await
}
