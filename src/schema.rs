// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        is_actif -> Bool,
        is_superuser -> Bool,
        is_staff -> Bool,
        is_test -> Bool,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
