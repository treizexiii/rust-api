#![allow(unused)]

mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod web;
mod utils;

pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;
use model::ticket::TicketRepository;

use crate::ctx::Ctx;
use crate::log::log_request;
use crate::model::DbContext;
use crate::web::routes_static::{route_hello, serve_dir};
use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::Response;
use axum::routing::{get, get_service, Route};
use axum::{
    middleware,
    response::{Html, IntoResponse},
    routing, Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::log::{debug, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use crate::web::middlewares::main_response_mapper;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // -- FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize managers
    let db = DbContext::new().await?;

    // Initialize controllers
    let tickets = TicketRepository::new().await?;

    // let routes_api = web::routes_tickets::routes(tickets.clone())
    //     .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    // register routes
    let routes_all = Router::new()
        .merge(route_hello())
        .merge(web::routes_login::routes(db.clone()))
        // .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        // .layer(middleware::from_fn_with_state(
        //     tickets.clone(),
        //     web::mw_auth::mw_ctx_resolver,
        // ))
        .layer(CookieManagerLayer::new())
        .fallback_service(serve_dir());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("{:<12} - {addr}\n", "LISTENING");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
