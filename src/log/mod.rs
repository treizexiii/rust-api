use std::time::{SystemTime, UNIX_EPOCH};
use axum::http::{Method, Uri};
use log::info;
use serde_json::{json, Value};
use serde_with::serde_derive::Serialize;
use serde_with::skip_serializing_none;
use tracing::log::debug;
use uuid::Uuid;
use crate::ctx::Ctx;
use crate::Result;
use crate::web::{self, ClientError};
use crate::web::rpc::RpcInfo;

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String, // ISO8601

    user_id: Option<i64>,

    req_path: String,
    req_method: String,

    rpc_id: Option<String>,
    rpc_method: Option<String>,

    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

pub async fn log_request(
    uuid: Uuid,
    method: Method,
    uri: Uri,
    rpc_info: Option<&RpcInfo>,
    ctx: Option<Ctx>,
    service_error: Option<&web::Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok().
        and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        req_path: uri.to_string(),
        req_method: method.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        rpc_id: rpc_info.and_then(|rpc| rpc.id.as_ref().map(|id| id.to_string())),
        rpc_method: rpc_info.map(|rpc| rpc.method.to_string()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    info!("LOG_REQUEST: \n{}", json!(log_line));

    Ok(())
}
