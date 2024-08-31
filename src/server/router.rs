use std::{
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};

use axum::{
    extract::MatchedPath,
    http::{HeaderValue, Request},
    middleware::from_fn_with_state,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE, COOKIE},
    Method, StatusCode,
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
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

use super::{api_docs::ApiDoc, auth::jwt_middleware, AppState};
use crate::{
    companies::routes as companies_routes,
    countries::routes as countries_routes,
    currencies::routes as currencies_routes,
    dictionary::routes as dictionary_routes,
    exchanges::routes as exchanges_routes,
    industries::routes as industries_routes,
    sectors::routes as sectors_routes,
    transactions::{accounts_routes, transactions_routes},
    users::routes as users_routes,
};

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
        )
        .layer(std_cors());

    Router::new()
        .route("/", get(home))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/:version/", api_routes(state.clone()))
        .fallback(error_404)
        .layer(middleware)
        .with_state(state)
}

#[derive(Clone, Default)]
struct MyMakeRequestId {
    counter: Arc<AtomicU64>,
}

use std::sync::atomic::Ordering;

impl MakeRequestId for MyMakeRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        self.counter
            .fetch_add(1, Ordering::SeqCst)
            .to_string()
            .parse()
            .ok()
            .map(|r| RequestId::new(r))
    }
}

fn std_cors() -> CorsLayer {
    CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        // allow requests from any origin
        .allow_origin(Any)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(countries_routes(state.clone()))
        .merge(companies_routes(state.clone()))
        .merge(exchanges_routes(state.clone()))
        .merge(currencies_routes(state.clone()))
        .merge(industries_routes(state.clone()))
        .merge(sectors_routes(state.clone()))
        .merge(transactions_routes(state.clone()))
        .merge(accounts_routes(state.clone()))
        .merge(dictionary_routes(state.clone()))
        .layer(from_fn_with_state(state.clone(), jwt_middleware))
        .merge(users_routes(state.clone())) //TODO: implement better auth
        .with_state(state)
}

async fn home() -> Response {
    (StatusCode::OK, Html("<h1>Welcome to Elerem</h1>")).into_response()
}

async fn error_404() -> Response {
    (StatusCode::NOT_FOUND, Html("<h1>Nothing to see here</h1>")).into_response()
}
