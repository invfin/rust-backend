use std::{
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};

use minijinja::context;
use serde::{Deserialize, Serialize};

use axum::{
    extract::MatchedPath,
    http::{HeaderValue, Request},
    middleware::from_fn_with_state,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE, COOKIE},
    http, Method, StatusCode,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    normalize_path::NormalizePathLayer,
    request_id::{MakeRequestId, RequestId},
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    timeout::TimeoutLayer,
    trace::{
        DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse,
        TraceLayer,
    },
    LatencyUnit, ServiceBuilderExt,
};
use tracing::{info_span, Level};

use super::{auth::jwt_middleware, AppState};
use crate::{
    companies::handlers::{
        companies::{routes as companies_routes, ApiDoc as ApiDocCompanies},
        routes::{routes as companies_helpers_routes, ApiDoc as ApiDocCompaniesHelpers},
    },
    dictionary::handlers::{routes as dictionary_routes, ApiDoc as ApiDocDictionary},
    server::ErrorMessage,
    transactions::handlers::{
        expenses_routes, incomes_routes, investments_routes, ApiDocExpenses, ApiDocIncomes,
        ApiDocInvestments,
    },
    users::handlers::{routes as users_routes, ApiDoc as UsersDoc},
};

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_scalar::{Scalar, Servable as ScalarServable};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    security(("jwt" = ["*"])),
    servers(
        (url = "http://{domain}:{port}/api/{version}", description = "Local server",
            variables(
                ("domain" = (default = "127.0.0.1", description = "Default domain for API")),
                ("port" = (default = "8000", enum_values("8000", "5000", "3030"), description = "Supported ports for API")),
                ("version" = (default = "v1", enum_values("v1"), description = "Supported versions for API")),
            )
        )),
    nest(
        (path = "/", api = ApiDocUsers, tags = ["Users"]),
        (path = "/", api = ApiDocExpenses, tags = ["Expenses"]),
        (path = "/", api = ApiDocIncomes, tags = ["Incomes"]),
        (path = "/", api = ApiDocInvestments, tags = ["Investments"]),

        (path = "/", api = ApiDocDictionary, tags = ["Dictionary"]),
        (path = "/", api = ApiDocCompanies, tags = ["Companies"]),
        (path = "/", api = ApiDocExchanges, tags = ["Exchanges"]),
        (path = "/", api = ApiDocIndustries, tags = ["Industries"]),
        (path = "/", api = ApiDocPeriods, tags = ["Periods"]),
        (path = "/", api = ApiDocSectors, tags = ["Sectors"]),
    ),
    components(
        schemas(ErrorMessage),
        responses(ErrorMessage)
    ),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
