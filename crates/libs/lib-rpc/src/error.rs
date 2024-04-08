use derive_more::From;
use lib_core::model;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

// region:     Error

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    RpcMethodUnknown(String),
    RpcMissingParams {
        rpc_method: String,
    },
    RpcFailJsonParams {
        rpc_method: String,
    },

    // -- Modules
    #[from]
    Model(model::Error),

    // -- External Modules
    #[from]
    SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

// region:     Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}
// endregion:  Error Boilerplate

// endregion:  Error
