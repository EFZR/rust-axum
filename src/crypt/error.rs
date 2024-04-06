use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Clone, Debug, Serialize)]
pub enum Error {
    // -- Key
    KeyFailHmac,

    // -- Pwd
    PwdNotMatching,

    // -- Token
    TokenInvalidFormat,
    TokenCannotDecodeIdent,
    TokenCannotDecodeExp,
    TokenSignatureNotMatching,
    TokenExpNotIso,
    TokenExpired,
}

// region:          --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion:       --- Error Boilerplate
