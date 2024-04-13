// region:      --- Modules

mod error;
mod resources;
mod rpc_params;
mod rpc_result;
mod rpcs;

pub mod router;

pub use self::error::{Error, Result};
pub use self::resources::RpcResources;

// endregion:   --- Modules
