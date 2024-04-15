//! This is a prelude for all .._rpc modules to avoid redundant imports.
//! NOTE: This is only for the `rpcs` module and sub-modules.

pub use crate::generate_common_rpc_fns;
pub use crate::router::RpcRouter;
pub use crate::rpc_params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
pub use crate::rpc_result::DataRpcResult;
pub use crate::rpc_router;
pub use crate::Result;
pub use lib_core::ctx::Ctx;
pub use lib_core::model::ModelManager;
pub use paste::paste;
