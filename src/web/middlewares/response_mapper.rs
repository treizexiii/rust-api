use std::sync::Arc;
use axum::http::{Method, Uri};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::{json, to_value};
use tracing::debug;
use uuid::Uuid;
use crate::ctx::Ctx;
use crate::log::log_request;
use crate::web;
use crate::web::rpc::RpcInfo;

pub async fn mw_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!("{:<12} - main_response_mapper - {res:?}", "RES_MAPPER");

    let uuid = Uuid::new_v4();

    let rpc_info = res.extensions().get::<RpcInfo>();

    let service_error = res.extensions().get::<Arc<web::Error>>().map(Arc::as_ref);
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response =
        client_status_error
            .as_ref()
            .map(|&(ref status_code, ref client_error)| {
                let client_error = to_value(client_error).ok();
                let message = client_error.as_ref().and_then(|v| v.get("message"));
                let detail = client_error.as_ref().and_then(|v| v.get("detail"));
                let client_error_body = json!({
                    "id": rpc_info.as_ref().map(|rpc| rpc.id.clone()),
                    "error": {
                        "message": message,
                        "data": {
                            "req_uuid": uuid.to_string(),
                            "detail": detail
                        }
                    }
                });
                debug!("CLIENT_ERROR_BODY: {client_error_body}");

                (*status_code, Json(client_error_body)).into_response()
            });

    let client_error = client_status_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, rpc_info, ctx, service_error, client_error).await;

    debug!("END OF REQUEST\n");
    error_response.unwrap_or(res)
}
