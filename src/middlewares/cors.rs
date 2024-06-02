use http::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{http, Method};
use tower_http::cors::{Any, CorsLayer};

pub fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_origin(Any)
}

pub fn post_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_origin(Any)
}
