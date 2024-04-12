// region:     Error

use super::scheme;
use derive_more::From;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, From)]
pub enum Error {
    PwdWithSchemeFailedParse,
    FailSpawnBlockForHash,
    FailSpawnBlockForValidate,

    // -- Modules
    #[from]
    Scheme(scheme::Error),
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
