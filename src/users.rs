use axum::{extract::ConnectInfo, routing::post, Json, Router};
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    Connection, ExpressionMethods, Insertable, OptionalExtension, PgConnection, Queryable,
    RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};

use crate::{
    countries::{get_country_id, CountryIndexes},
    currencies::get_currency_from_country,
    db::schema::{profiles, users},
    server::{create_token, AppError, AppJson, AppResult, AppState},
};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use utoipa::OpenApi;
use utoipa::{ToResponse, ToSchema};

use std::net::SocketAddr;

use maxminddb::geoip2;

#[derive(OpenApi)]
#[openapi(
    paths(login, register),
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

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(AppError::ErrorHashingPassword)?
        .to_string())
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
    fn default() -> Self {
        Self {
            username: String::new(),
            email: String::new(),
            is_active: false,
            is_superuser: false,
            is_staff: false,
            is_test: false,
            password: String::new(),
        }
    }
    fn new(query_params: RegisterPayload) -> Result<Self, AppError> {
        Ok(Self {
            username: query_params.username,
            email: query_params.email,
            is_active: true,
            password: hash_password(&query_params.password)?,
            ..Self::default()
        })
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
struct UserCore {
    id: i64,
    username: String,
    email: String,
    password: String,
    is_active: bool,
    is_superuser: bool,
    is_staff: bool,
    is_test: bool,
}
impl UserCore {
    fn role(&self) -> String {
        "admin".to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Profile {
    user_id: i64,
    currency_id: i64,
    country_id: i64,
    image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToResponse, ToSchema)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    #[serde(skip_serializing)]
    id: i64,
    role: String,
    username: String,
    email: String,
    image: Option<String>,
    currency_id: i64,
    token: String,
}

impl LoginResponse {
    fn new(user: &UserCore, profile: &Profile) -> Self {
        Self {
            id: user.id,
            role: user.role(),
            username: user.username.clone(),
            email: user.email.clone(),
            image: profile.image.clone(),
            currency_id: profile.currency_id,
            token: String::new(),
        }
    }
    async fn set_token(&mut self, state: &AppState) -> Result<&Self, AppError> {
        self.token = create_token(self.id, &self.role, state).await?;
        Ok(self)
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
    state
        .db_write()
        .await?
        .interact(move |conn| {
            let user = users::table
                .filter(users::email.eq(payload.email))
                .select(UserCore::as_select())
                .first(conn)
                .optional()
                .map_err(AppError::DatabaseQueryError)?
                .ok_or(AppError::DoesNotExist)?;
            let parsed_hash =
                PasswordHash::new(&user.password).map_err(AppError::ErrorHashingPassword)?;
            Argon2::default()
                .verify_password(payload.password.as_bytes(), &parsed_hash)
                .map_err(AppError::WrongPassword)?;
            let profile = profiles::table
                .filter(profiles::user_id.eq(user.id))
                .select(Profile::as_select())
                .first(conn)
                .map_err(AppError::DatabaseQueryError)?;
            Ok(Json(LoginResponse::new(&user, &profile)))
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)?
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
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    AppJson(payload): AppJson<RegisterPayload>,
) -> AppResult<LoginResponse> {
    let country_iso = get_country_from_ip(&state.ips_database, &addr)?.to_owned();
    let mut user: LoginResponse = state
        .db_write()
        .await?
        .interact(move |conn| {
            conn.transaction::<LoginResponse, AppError, _>(|conn| {
                let user = diesel::insert_into(users::table)
                    .values(NewUser::new(payload)?)
                    .returning(UserCore::as_returning())
                    .get_result(conn)
                    .map_err(AppError::DatabaseQueryError)?;
                let profile = create_profile(user.id, &country_iso, conn)?;
                Ok(LoginResponse::new(&user, &profile))
            })
        })
        .await
        .map_err(AppError::DatabaseConnectionInteractError)??;
    user.set_token(&state).await?;
    Ok(Json(user))
}

fn get_country_from_ip<'a>(
    ips_database: &'a maxminddb::Reader<Vec<u8>>,
    addr: &'a SocketAddr,
) -> Result<&'a str, AppError> {
    ips_database
        .lookup::<geoip2::City>(addr.ip())
        .map_err(AppError::IpError)?
        .country
        .ok_or(AppError::IpDataNotFound)?
        .iso_code
        .ok_or(AppError::IpDataNotFound)
}

fn create_profile(
    user_id: i64,
    country_iso: &str,
    conn: &mut PgConnection,
) -> Result<Profile, AppError> {
    let country_id = get_country_id(CountryIndexes::Iso(country_iso), conn)?;
    let currency_id = get_currency_from_country(country_id, conn)?;
    let profile = Profile {
        user_id,
        currency_id,
        country_id,
        image: None,
    };
    diesel::insert_into(profiles::table)
        .values(&profile)
        .execute(conn)
        .map_err(AppError::DatabaseQueryError)?;
    Ok(profile)
}
