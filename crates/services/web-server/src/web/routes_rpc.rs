use crate::web::mw_auth::CtxW;
use crate::web::Result;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use serde_json::{json, Value};
use tracing::debug;

/// RPC basic information containing the rpc request
/// id and method for additional logging purposes.
#[derive(Debug)]
pub struct RpcInfo {
    pub id: Option<Value>,
    pub method: String,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_axum_handler))
        .with_state(mm)
}

async fn rpc_axum_handler() -> Response {
    todo!()
}
