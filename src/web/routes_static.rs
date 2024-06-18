use axum::extract::{Path, Query};
use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::{any_service, get, get_service, MethodRouter};
use serde::Deserialize;
use tower_http::services::ServeDir;
use tracing::log::debug;
use crate::config;

// pub fn routes_static() -> Router {
//     Router::new()
//         .nest_service("/", get_service(ServeDir::new("./")))
// }

pub fn serve_dir() -> MethodRouter {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Ressource not found")
    }

    any_service(
        ServeDir::new(&config().WEB_FOLDER)
            .not_found_service(handle_404.into_service())
    )
}

pub fn route_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    debug!("{:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    debug!("{:<12} - handler_hello - {name}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
