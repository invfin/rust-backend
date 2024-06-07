use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl}, ExpressionMethods, Insertable, Queryable, RunQueryDsl,
    Selectable, SelectableHelper, OptionalExtension
};
use serde::{Deserialize, Serialize};
use axum::Json;

use crate::{db::schema::users, server::{create_token, AppError,AppState, AppResult}};


#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: String,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: String,
    pub is_actif: bool,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_test: bool,
    pub password: String,
}

impl<'a> NewUser<'a> {
    pub fn new(query_params: RegisterPayload) -> Self {
        Self {
            username: query_params.username,
            first_name: "",
            last_name: "",
            email: query_params.email,
            is_actif: true,
            is_superuser: false,
            is_staff: false,
            is_test: false,
            password: query_params.password,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LoginUser {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub token: String,
}

impl LoginResponse{
    async fn new(user:&LoginUser, state: &AppState)->AppResult<LoginResponse>{
        Ok(Json(Self{
            id: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            token: create_token(user.id, "admin".to_owned(), state).await?
        }))
    }
}

pub async fn login(state: AppState, Json(payload): Json<LoginPayload>) -> AppResult<LoginResponse> {
    let conn =  state.db_write().await?;
    let user = conn
        .interact(move |conn| {
            users::table
                .filter(users::username.eq(payload.username))
                .filter(users::password.eq(payload.password))
                .select((users::id, users::username, users::email))
                .first::<LoginUser>(conn)
                .optional()
                .map_err(AppError::DatabaseQueryError)
                
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??.ok_or(AppError::DoesNotExist)?;
        LoginResponse::new(&user, &state).await
}

pub async fn register(state: AppState, Json(payload): Json<RegisterPayload>) -> AppResult<LoginResponse> {
    let conn = state.db_write().await?;
    let user = conn
        .interact(move |conn| {
            diesel::insert_into(users::table)
                .values(NewUser::new(payload))
                .returning(LoginUser::as_returning())
                .get_result(conn)
                .map_err(AppError::DatabaseQueryError)
        })
        .await.map_err(AppError::DatabaseConnectionInteractError)??;
    LoginResponse::new(&user, &state).await
}
