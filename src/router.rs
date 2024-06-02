// use super::middlewares::{get_cors, post_cors};

use crate::middlewares::auth::{authorize, protected};
use crate::middlewares::cors::post_cors;

use crate::responses::not_found;
use crate::users::facades::{create_user, list_users};
use crate::{versioning::Version, App, AppState};
use axum::extract::{DefaultBodyLimit, Path};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post, put};
use axum::Router;
use axum::{
    extract::{Form, MatchedPath, Query},
    http::{Request, StatusCode},
    response::{Html, Redirect, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    trace::{
        DefaultOnBodyChunk, DefaultOnEos, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse,
        TraceLayer,
    },
    LatencyUnit,
};
use tracing::{info_span, Level};

pub fn build_router(state: AppState) -> Router<()> {
    Router::new()
        .route("/", get(home))
        .nest("/api/:version/", api_routes(state.clone()))
        .route("/protected", get(protected))
        .route("/authorize", post(authorize))
        .layer(post_cors())
        .fallback(error_404)
        // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
        // It provides good defaults but is also very customizable.
        //
        // See https://docs.rs/tower-http/latest/tower_http/trace/index.html for more details.
        //
        // If you want to customize the behavior using closures here is how.
        .layer(
            ServiceBuilder::new().layer(
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
            ),
        )
        .with_state(state)
}

fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user).get(list_users))
        .with_state(state)
}

pub async fn home() -> Response {
    (StatusCode::OK, Html("<h1>Welcome to Elerem</h1>")).into_response()
}

pub async fn error_404() -> Response {
    (StatusCode::NOT_FOUND, Html("<h1>Nothing to see here</h1>")).into_response()
}
