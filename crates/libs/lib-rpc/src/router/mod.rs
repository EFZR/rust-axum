//! rpc::router module provides the type and implementation for
//! json rpc routing.
//!
//! It has the following constructs:
//!
//! - `RpcRouter` holds the HashMap of `method_name: Box<dyn RpcHandlerWrapperTrait>`.
//! - `RpcHandler` trait is implemented for any async function that, with
//!   `(S1, S2, ...[impl IntoParams])`, returns `web::Result<Serialize>` where S1, S2, ... are
//!    types that implement `FromResources` (see router/from_resources.rs and src/resources.rs).
//! - `IntoParams` is the trait to implement to instruct how to go from `Option<Value>` json-rpc params
//!   to the handler's param types.
//! - `IntoParams` has a default `into_params` implementation that will return an error if the params are missing.
//!
//! ```
//! #[derive(Deserialize)]
//! pub struct ParamsIded {
//!   id: i64,
//! }
//!
//! impl IntoParams for ParamsIded {}
//! ```
//!
//! - For custom `IntoParams` behavior, implement the `IntoParams::into_params` function.
//! - Implementing `IntoDefaultParams` on a type that implements `Default` will auto-implement `IntoParams`
//!   and call `T::default()` when the params `Option<Value>` is None.
//!

// region:      --- Modules

mod from_resources;
mod into_params;
mod rpc_handler;
mod rpc_handler_wrapper;

pub use from_resources::FromResources;
pub use into_params::{IntoDefaultParams, IntoParams};
pub use rpc_handler::RpcHandler;
pub use rpc_handler_wrapper::{RpcHandlerWrapper, RpcHandlerWrapperTrait};

use crate::RpcResources;
use crate::{Error, Result};
use futures::Future;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::pin::Pin;

// endregion:   --- Modules

/// The raw JSON-RPC request object, serving as the foundation for RPC routing.
#[derive(Deserialize)]
pub struct RpcRequest {
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

pub type PinFutureValue = Pin<Box<dyn Future<Output = Result<Value>> + Send>>;
