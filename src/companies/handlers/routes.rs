use crate::server::AppState;

use super::{
    exchanges::{
        create_exchange, delete_exchange, read_exchange, update_exchange, ApiDoc as ApiDocExchanges,
    },
    industries::{
        create_industry, delete_industry, read_industry, update_industry,
        ApiDoc as ApiDocIndustries,
    },
    periods::{create_period, delete_period, read_period, update_period, ApiDoc as ApiDocPeriods},
    sectors::{create_sector, delete_sector, read_sector, update_sector, ApiDoc as ApiDocSectors},
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/", api = ApiDocExchanges, tags = ["Exchanges"]),
        (path = "/", api = ApiDocIndustries, tags = ["Industries"]),
        (path = "/", api = ApiDocPeriods, tags = ["Periods"]),
        (path = "/", api = ApiDocSectors, tags = ["Sectors"]),
    ),
)]
pub struct ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Periods
        .route("/periods", post(create_period))
        .route(
            "/periods/:id",
            get(read_period).put(update_period).delete(delete_period),
        )
        // Exchanges
        .route("/exchanges", post(create_exchange))
        .route(
            "/exchanges/:id",
            get(read_exchange)
                .put(update_exchange)
                .delete(delete_exchange),
        )
        // Sectors
        .route("/sectors", post(create_sector))
        .route(
            "/sectors/:id",
            get(read_sector).put(update_sector).delete(delete_sector),
        )
        // Industries
        .route("/industries", post(create_industry))
        .route(
            "/industries/:id",
            get(read_industry)
                .put(update_industry)
                .delete(delete_industry),
        )
        .with_state(state)
}
