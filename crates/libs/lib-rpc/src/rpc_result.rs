//! The `lib_rpc::response` module normalizes the JSON-RPC `.result` format for various
//! JSON-RPC APIs.
//!
//! The primary type is the simple `DataRpcResult`, which contains only `data` property.
//!
//! Notes:
//!
//!     -   In the future, we may introduce types like `DataRpcResult` that includes metadata
//!         about the returned list data (e.g., pagination information).
//!     -   Although the struct is named with `Result`, it is not a typical Rust result. Instead,
//!         it represents the `.result` property of a JSON-RPC response.
//!

use serde::Serialize;

#[derive(Serialize)]
pub struct DataRpcResult<T>
where
    T: Serialize,
{
    data: T,
}

// The `From` trait is used to create an instance of a type from another type.
impl<T> From<T> for DataRpcResult<T>
where
    T: Serialize,
{
    // By implementing the `From` trait for `DataRpcResult<T>`, we can use the `from` method to create an instance of `DataRpcResult<T>` from a value of type `T`.
    // This is assuming that the value has an implementation for `T`.
    // Usage: `DataRpcResult::from(value)` or `let result: DataRpcResult<_> = value.into()`
    fn from(value: T) -> Self {
        Self { data: value }
    }
}