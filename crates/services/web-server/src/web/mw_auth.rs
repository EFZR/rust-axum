// region:     Modules

use serde::Serialize;

// endregion:  Modules

// region:     CtxExtError

pub type Result<T> = std::result::Result<T, CtxExtError>;

#[derive(Debug, Serialize)]
pub enum CtxExtError {
    
}

// region:     CtxExtError Boilerplate
impl core::fmt::Display for CtxExtError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for CtxExtError {}
// endregion:  CtxExtError Boilerplate

// endregion:  CtxExtError