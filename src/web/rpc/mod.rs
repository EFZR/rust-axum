// region:      --- Modules

mod params;
mod task_rpc;

use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::web::rpc::task_rpc::{crate_task, delete_task, list_tasks, update_task};
use crate::web::{Error, Result};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum::{extract::State, response::Response, Json};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;

// endregion:   --- Modules

// region:      --- RPC Types

/// The raw JSON-RPC request object, serving as the foundation for RPC routing.
#[derive(Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

/// RPC basic information holding the id and method for further logging.
#[derive(Debug, Clone)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

// endregion:   --- RPC Types

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

macro_rules! exec_rpc_fn {
    // With params
    ($rpc_fn:expr, $ctx:expr, $mm: expr, $rpc_params:expr) => {{
        let rpc_fn_name = stringify!($rpc_fn);

        let params = $rpc_params.ok_or(Error::RpcMissingParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;

        let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
            rpc_method: rpc_fn_name.to_string(),
        })?;

        $rpc_fn($ctx, $mm, params).await.map(to_value)??
    }};

    // Without params
    ($rpc_fn:expr, $ctx:expr, $mm: expr) => {
        $rpc_fn($ctx, $mm).await.map(to_value)??
    };
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    ctx: Ctx,
    Json(rpc_req): Json<RpcRequest>,
) -> Response {
    // -- Create the RPC Info to be set to the response.extensions.
    let rpc_info = RpcInfo {
        id: rpc_req.id.clone(),
        method: rpc_req.method.clone(),
    };

    // -- Exec & Store RpcInfo in response.
    let mut res = _rpc_handler(mm, ctx, rpc_req).await.into_response();
    res.extensions_mut().insert(rpc_info);

    res
}

async fn _rpc_handler(mm: ModelManager, ctx: Ctx, rpc_req: RpcRequest) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result_json: Value = match rpc_method.as_str() {
        // -- Task RPC methods.
        "list_tasks" => exec_rpc_fn!(list_tasks, ctx, mm, rpc_params),
        "create_task" => exec_rpc_fn!(crate_task, ctx, mm, rpc_params),
        "update_task" => exec_rpc_fn!(update_task, ctx, mm, rpc_params),
        "delete_task" => exec_rpc_fn!(delete_task, ctx, mm, rpc_params),

        // -- Fallback as Err.
        _ => return Err(Error::RpcMethodUnknown(rpc_method)),
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_json
    });

    Ok(Json(body_response))
}
