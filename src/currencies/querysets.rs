use crate::{db::schema::currencies_countries_m2m, server::AppError};

use diesel::prelude::*;

pub fn get_currency_from_country(
    country_id: i64,
    conn: &mut PgConnection,
) -> Result<i64, AppError> {
    currencies_countries_m2m::table
        .filter(currencies_countries_m2m::country_id.eq(country_id))
        .select(currencies_countries_m2m::currency_id)
        .first(conn)
        .map_err(AppError::DatabaseQueryError)
}
