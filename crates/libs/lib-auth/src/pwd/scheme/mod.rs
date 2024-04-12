// region:     --- Modules

mod error;
mod scheme_01;
mod scheme_02;

pub use self::error::{Error, Result};

use crate::pwd::ContentToHash;

// endregion:  --- Modules

pub const DEFAULT_SCHEME: &str = "02";

#[derive(Debug)]
pub enum SchemeStatus {
    Ok,       // The pwd uses the latest scheme. All good.
    Outdated, // The pwd uses an old scheme.
}

pub trait Scheme {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String>;

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()>;
}

// Read Note 1) below
enum SchemeDispatcher {
    Scheme01(scheme_01::Scheme01),
    Scheme02(scheme_02::Scheme02),
}

impl Scheme for SchemeDispatcher {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        match self {
            SchemeDispatcher::Scheme01(scheme) => scheme.hash(to_hash),
            SchemeDispatcher::Scheme02(scheme) => scheme.hash(to_hash),
        }
    }

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()> {
        match self {
            SchemeDispatcher::Scheme01(scheme) => scheme.validate(to_hash, pwd_ref),
            SchemeDispatcher::Scheme02(scheme) => scheme.validate(to_hash, pwd_ref),
        }
    }
}

pub fn get_scheme(scheme_name: &str) -> Result<impl Scheme> {
    match scheme_name {
        "01" => Ok(SchemeDispatcher::Scheme01(scheme_01::Scheme01)),
        "02" => Ok(SchemeDispatcher::Scheme02(scheme_02::Scheme02)),
        _ => Err(Error::SchemeNotFound(scheme_name.to_string())),
    }
}

// 1)   This pattern, known as "enum dispatch", provides an alternative to `Box<dyn Scheme>` for dynamic dispatch.
//      By using this pattern, we can achieve static dispatch, which is both efficient and neatly organized.
