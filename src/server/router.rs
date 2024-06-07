use std::{time::Duration, sync::{atomic::AtomicU64,Arc}};

use axum::{extract::MatchedPath, http::{HeaderValue,Request}, routing::{get, post}, Router};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE};
use tower::ServiceBuilder;
use tower_http::{
    timeout::TimeoutLayer,
    normalize_path::NormalizePathLayer,
    request_id::{MakeRequestId, RequestId},
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    ServiceBuilderExt,
    trace::{
        DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse,
        TraceLayer,
    },
    LatencyUnit,
};
use tracing::{info_span, Level};

use crate::{users::handlers::login, AppState};
use super::{auth::{jwt_middleware, protected}, routes::{api_routes, error_404, home}};


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

pub fn get_router(state: AppState) -> Router<()> {
    let sensitive_headers: Arc<[_]> = vec![AUTHORIZATION, COOKIE].into();
    // Build our middleware stack
    let middleware = ServiceBuilder::new()
    .layer(NormalizePathLayer::trim_trailing_slash())
        // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
        .layer(SetSensitiveRequestHeadersLayer::from_shared(sensitive_headers.clone()))
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
        // // Authorize requests using a token
        // .layer(ValidateRequestHeaderLayer::bearer("passwordlol"))
        // // Accept only application/json, application/* and */* in a request's ACCEPT header
        // .layer(ValidateRequestHeaderLayer::accept("application/json"))
        ;

    Router::new()
        .route("/", get(home))
        .route("/login", post(login))
        .nest("/api/:version/", api_routes(state.clone()))
        .route("/protected", get(protected))
        .fallback(error_404)
        .layer(middleware)
        .with_state(state)
}
