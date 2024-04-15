use crate::router::RpcRouter;

mod macro_utils;
mod prelude;
mod task_rpc;

pub fn all_rpc_routers() -> RpcRouter {
    RpcRouter::new().extends(task_rpc::rpc_router())
}
