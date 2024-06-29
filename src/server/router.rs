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
    companies::handlers::{routes as companies_routes, ApiDoc as CompaniesDoc},
    server::{AppError, AppJson, ErrorMessage},
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
                ("version" = (default = "v1", description = "Supported versions for API")),
            )
        )),
    nest(
        (path = "/", api = UsersDoc, tags = ["Users"]),
        (path = "/", api = CompaniesDoc, tags = ["Companies"]),
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

// A `MakeRequestId` that increments an atomic counter
#[derive(Clone, Default)]
struct MyMakeRequestId {
    counter: Arc<AtomicU64>,
}

use std::sync::atomic::Ordering;

impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, request: &Request<B>) -> Option<RequestId> {
        let request_id = self
            .counter
            .fetch_add(1, Ordering::SeqCst)
            .to_string()
            .parse()
            .unwrap();

        Some(RequestId::new(request_id))
    }
}

fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_origin(Any)
}

fn post_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(Any)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(companies_routes(state.clone()))
        .layer(from_fn_with_state(state.clone(), jwt_middleware))
        .merge(users_routes(state.clone()))
        .layer(post_cors())
        // .layer(AsyncRequireAuthorizationLayer::new(MyAuth))
        .with_state(state)
}

async fn home() -> Response {
    (StatusCode::OK, Html("<h1>Welcome to Elerem</h1>")).into_response()
}

async fn error_404() -> Response {
    (StatusCode::NOT_FOUND, Html("<h1>Nothing to see here</h1>")).into_response()
}

#[derive(Serialize, Deserialize)]
struct Site<'a> {
    name: &'a str,
    url: &'a str,
    img: &'a str,
}

async fn handler_index(state: AppState) -> Result<Html<String>, StatusCode> {
    let template = state.templates.get_template("home").unwrap();
    let some_example_entries = vec![
        Site {
            name: "portainer",
            url: "http://portainer.raspi",
            img: "https://www.portainer.io/hubfs/portainer-logo-black.svg",
        },
        Site {
            name: "pihole",
            url: "http://pihole.raspi/admin",
            img: "https://camo.githubusercontent.com/5e788319ebef8b0c2bd64b8284690fabc29abdf2d3e00ff84cf05d0027e595a9/68747470733a2f2f70692d686f6c652e6769746875622e696f2f67726170686963732f566f727465782f566f727465785f776974685f746578742e706e67",
        },
        Site {
            name: "nginx",
            url: "http://nginx.raspi",
            img: "https://nginxproxymanager.com/logo.svg"
        },
        Site {
            name: "cinema",
            url: "http://cinema.raspi",
            img: "https://static.vecteezy.com/system/resources/previews/012/262/720/non_2x/creative-cinema-logo-design-greeting-card-banner-poster-illustration-vector.jpg"
        },
    ];
    let rendered = template
        .render(context! {
            title => "Home",
            entries => some_example_entries,
        })
        .unwrap();

    Ok(Html(rendered))
}

pub fn get_router(state: AppState) -> Router<()> {
    let sensitive_headers: Arc<[_]> = vec![AUTHORIZATION, COOKIE].into();
    // Build our middleware stack
    let middleware = ServiceBuilder::new()
        .layer(NormalizePathLayer::trim_trailing_slash())
        // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
        .layer(SetSensitiveRequestHeadersLayer::from_shared(
            sensitive_headers.clone(),
        ))
        .set_x_request_id(MyMakeRequestId::default())
        // Add high level tracing/logging to all requests
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros)
                        .include_headers(true),
                )
                .on_body_chunk(DefaultOnBodyChunk::new())
                .on_eos(DefaultOnEos::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::INFO)),
        )
        .sensitive_response_headers(sensitive_headers)
        // Set a timeout
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // Compress responses
        .compression()
        .propagate_x_request_id()
        // Set a `Content-Type` if there isn't one already.
        .insert_response_header_if_not_present(
            CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );

    Router::new()
        .route("/", get(home))
        .route("/index", get(handler_index))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version/", api_routes(state.clone()))
        .fallback(error_404)
        .layer(middleware)
        .with_state(state)
}
