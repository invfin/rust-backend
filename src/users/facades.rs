use crate::db::schema::users;
use chrono::NaiveDateTime;
use diesel::{
    query_dsl::methods::FilterDsl, ExpressionMethods, Insertable, Queryable, RunQueryDsl,
    Selectable, SelectableHelper,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use axum::{
    async_trait,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    routing::get,
    Form, Json, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde_json::json;

use crate::server::{success, AppState, Version};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_actif: bool,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_test: bool,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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
    pub fn new(query_params: NewUserQueryParams) -> Self {
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
pub struct NewUserQueryParams {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct FilterUsers {
    pub is_actif: Option<bool>,
    pub is_superuser: Option<bool>,
    pub is_staff: Option<bool>,
    pub is_test: Option<bool>,
}

pub async fn list_users(
    _version: Version,
    app: AppState,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Query(payload): Query<FilterUsers>,
) -> impl IntoResponse {
    let conn = app.db_write().await.unwrap();
    let users_result = conn
        .interact(move |conn| {
            users::table
                .filter(users::is_actif.eq(payload.is_actif.unwrap_or(true)))
                .load::<User>(conn)
                .unwrap()
        })
        .await
        .unwrap();
    success(users_result).await
}
//https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
pub async fn create_user(
    _version: Version,
    app: AppState,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<NewUserQueryParams>,
) -> impl IntoResponse {
    let conn = app.db_write().await.unwrap();
    let user = conn
        .interact(move |conn| {
            // let auth = AuthCheck::only_cookie().check(&req, conn).unwrap();
            // let user = auth.user();
            let new_user = NewUser::new(payload);
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(conn)
                .expect("Error saving new user")
        })
        .await
        .unwrap();
    success(user).await
}

pub async fn create_user_form(
    app: AppState,
    Form(payload): Form<NewUserQueryParams>,
) -> impl IntoResponse {
    let conn = app.db_write().await.unwrap();
    let user = conn
        .interact(move |conn| {
            // let auth = AuthCheck::only_cookie().check(&req, conn).unwrap();
            // let user = auth.user();
            let new_user = NewUser::new(payload);
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(User::as_returning())
                .get_result(conn)
                .expect("Error saving new user")
        })
        .await
        .unwrap();
    success(user).await
}
