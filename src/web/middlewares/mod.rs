use axum::http::{Method, StatusCode, Uri};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use tracing::debug;
use uuid::Uuid;
use super::error::Error;
use crate::ctx::Ctx;
use crate::log::log_request;

pub async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!("{:<12} - main_response_mapper - {res:?}", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response =
        client_status_error
            .as_ref()
            .map(|&(ref status_code, ref client_error)| {
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
