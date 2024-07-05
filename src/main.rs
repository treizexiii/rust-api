#![allow(unused)]

mod config;
mod error;
mod log;
mod model;
mod web;
mod ctx;

pub mod _dev_utils;

pub use self::error::{Error,Result};
pub use config::config;

use std::net::SocketAddr;
use axum::{response::{Html, IntoResponse}, routing, Router, middleware, Json};
use axum::extract::{Path, Query};
use axum::http::{Method, Uri};
use axum::response::Response;
use axum::routing::{get, get_service, Route};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::log::{debug, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use crate::ctx::Ctx;
use crate::log::log_request;
use crate::model::{ModelManager};
use crate::web::routes_static::{route_hello, serve_dir};

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
    let mm = ModelManager::new().await?;

    // Initialize controllers
    // let mc = ModelController::new().await?;
    //
    // let routes_api = web::routes_tickets::routes(mc.clone())
    //     .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    // register routes
    let routes_all = Router::new()
        .merge(route_hello())
        .merge(web::routes_login::routes())
        // .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        // .layer(middleware::from_fn_with_state(
        //     mc.clone(),
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

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response
) -> Response {
    debug!("{:<12} - main_response_mapper - {res:?}", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|&( ref status_code, ref client_error)| {
            let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});
            debug!("CLIENT_ERROR_BODY: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    debug!("END OF REQUEST\n");
    error_response.unwrap_or(res)
}
