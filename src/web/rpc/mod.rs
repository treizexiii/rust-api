use axum::extract::State;
use axum::{Json, Router};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use log::debug;
use crate::ctx::Ctx;
use crate::model::DbContext;
use crate::web::{Error, Result};
use crate::web::rpc::task_rpc::{create_task, delete_task, get_task, list_task, update_task};

mod task_rpc;

#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    id: i64,
    data: D,
}

#[derive(Deserialize)]
pub struct ParamsId {
    id: i64,
}

pub fn routes(db_context: DbContext) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(db_context)
}

async fn rpc_handler(State(db_context): State<DbContext>, ctx: Ctx, Json(rpc_req): Json<RpcRequest>) -> Response {
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };

    let mut response = _rpc_handler(ctx, db_context, rpc_req).await.into_response();

    response.extensions_mut().insert(rpc_info);

    response
}

#[derive(Debug, Clone)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

macro_rules! exec_rpc_fn {
    // without params
    ($rpc_fn:expr, $ctx:expr, $db_context:expr) => {
        $rpc_fn($ctx, $db_context).await.map(to_value)??
    };

    // with params
    ($rpc_fn:expr, $ctx:expr, $db_context:expr, $rpc_params: expr) => {{
        let rpc_fn_name = stringify!($rpc_fn);
        let params = $rpc_params.ok_or(Error::RpcMissingParams {
            rpc_method: rpc_fn_name.to_string()
        })?;
        let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
            rpc_method: rpc_fn_name.to_string()
        })?;
        $rpc_fn($ctx, $db_context, params).await.map(to_value)??
        }};
}

async fn _rpc_handler(ctx: Ctx, db_context: DbContext, request: RpcRequest) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params
    } = request;

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result_json: Value = match rpc_method.as_str() {
        "create_task" => exec_rpc_fn!(create_task, ctx, db_context, rpc_params),
        "list_task" => exec_rpc_fn!(list_task, ctx, db_context),
        "get_task" => exec_rpc_fn!(get_task, ctx, db_context, rpc_params),
        "update_task" => exec_rpc_fn!(update_task, ctx, db_context, rpc_params),
        "delete_task" => exec_rpc_fn!(delete_task, ctx, db_context, rpc_params),
        _ => return Err(Error::RpcMethodUnknown(rpc_method))
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_json
    });

    Ok(Json(body_response))
}
