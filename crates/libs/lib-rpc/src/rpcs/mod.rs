use crate::router::RpcRouter;

pub mod task_rpc;

mod macro_utils;
mod prelude;

pub fn all_rpc_router() -> RpcRouter {
    RpcRouter::new().extends(task_rpc::rpc_router())
}
