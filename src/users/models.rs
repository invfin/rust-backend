use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
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

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub is_actif: bool,
    pub is_superuser: bool,
    pub is_staff: bool,
    pub is_test: bool,
    pub password: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new(username: &'a str, email: &'a str, password: &'a str) -> Self {
        Self {
            username,
            first_name: "",
            last_name: "",
            email,
            is_actif: true,
            is_superuser: false,
            is_staff: false,
            is_test: false,
            password,
        }
    }
}
