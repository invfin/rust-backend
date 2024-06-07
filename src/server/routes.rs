use super::AppState;
use crate::users::handlers::{login, register};
use axum::{
    response::{Html, IntoResponse, Response},
    routing::post,
    Router,
};
use hyper::StatusCode;
use minijinja::context;
use serde::{Deserialize, Serialize};


pub fn api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        // .layer(AsyncRequireAuthorizationLayer::new(MyAuth))
        .with_state(state)
}

pub async fn home() -> Response {
    (StatusCode::OK, Html("<h1>Welcome to Elerem</h1>")).into_response()
}

pub async fn error_404() -> Response {
    (StatusCode::NOT_FOUND, Html("<h1>Nothing to see here</h1>")).into_response()
}

#[derive(Serialize, Deserialize)]
struct Site<'a> {
    name: &'a str,
    url: &'a str,
    img: &'a str,
}

pub async fn handler_index(state: AppState) -> Result<Html<String>, StatusCode> {
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
