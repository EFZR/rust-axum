// region:     --- Modules

mod error;
mod scheme_hmac_01;

pub use self::error::{Error, Result};

use crate::pwd::ContentToHash;

// endregion:  --- Modules

pub const DEFAULT_SCHEME: &str = "01";

pub trait Scheme {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String>;

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()>;
}

#[derive(Debug)]
pub enum SchemeStatus {
    Ok,       // The pwd uses the latest scheme. All good.
    Outdated, // The pwd uses an old scheme.
}

pub fn get_scheme(scheme_name: &str) -> Result<Box<dyn Scheme>> {
    match scheme_name {
        "01" => Ok(Box::new(scheme_hmac_01::SchemeHmac)),
        _ => Err(Error::SchemeNotFound(scheme_name.to_string())),
    }
}
