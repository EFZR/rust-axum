//! The pwd module is responsible for hashing and validating hashes.
//! It follows a multi-scheme hashing code design, allowing each
//! scheme to provide its own hashing and validation methods.
//!
//! Code Design Points:
//!
//! - Exposes two public async functions `hash_pwd(...)` and `validate_pwd(...)`
//! - `ContentToHash` represents the data to be hashed along with the corresponding salt.
//! - `SchemeStatus` is the result of `validate_pwd` which, upon successful validation, indicates
//!   whether the password needs to be re-hashed to adopt the latest scheme.
//! - Internally, the `pwd` module implements a multi-scheme code design with the `Scheme` trait.
//! - The `Scheme` trait exposes sync functions `hash` and `validate` to be implemented for each scheme.
//! - The two public async functions `hash_pwd(...)` and `validate_pwd(...)` call the scheme using
//!   `spawn_blocking` to ensure that long hashing/validation processes do not hinder the execution of smaller tasks.
//! - Schemes are designed to be agnostic of whether they are in an async or sync context, hence they are async-free.

// region:     Modules

mod error;
mod scheme;

pub use self::error::{Error, Result};
pub use scheme::SchemeStatus;

use crate::pwd::scheme::{get_scheme, Scheme, DEFAULT_SCHEME};
use lazy_regex::regex_captures;
use std::str::FromStr;
use uuid::Uuid;

// endregion:  Modules

// region:     Types

#[cfg_attr(test, derive(Clone))]
pub struct ContentToHash {
    pub content: String,
    pub salt: Uuid,
}

// endregion:  Types

// region:     Public Functions

/// Hash the password with the default scheme.
pub async fn hash_pwd(to_hash: ContentToHash) -> Result<String> {
    tokio::task::spawn_blocking(move || hash_for_scheme(DEFAULT_SCHEME, to_hash))
        .await
        .map_err(|_| Error::FailSpawnBlockForHash)?
}

/// Validate if an ContentToHash matches.
pub async fn validate_pwd(to_hash: ContentToHash, pwd_ref: &str) -> Result<SchemeStatus> {
    let PwdParts {
        scheme_name,
        hashed,
    } = pwd_ref.parse()?;

    // Note: We do first, so that we do not have to clonse the scheme_name.
    let scheme_status = if scheme_name == DEFAULT_SCHEME {
        SchemeStatus::Ok
    } else {
        SchemeStatus::Outdated
    };

    // Note: The `validate` function could be time-consuming depending on the complexity of the hashing algorithm used.
    //       Therefore, we use `spawn_blocking` to prevent this potentially long-running operation from blocking the rest of the system.
    tokio::task::spawn_blocking(move || validate_for_scheme(&scheme_name, to_hash, &hashed))
        .await
        .map_err(|_| Error::FailSpawnBlockForValidate)??;

    Ok(scheme_status)
}

// endregion:  Public Functions

// region:      --- Privates

fn hash_for_scheme(scheme_name: &str, to_hash: ContentToHash) -> Result<String> {
    let scheme = get_scheme(scheme_name)?;

    let pwd_hashed = scheme.hash(&to_hash)?;

    Ok(format!("#{scheme_name}#{pwd_hashed}"))
}

fn validate_for_scheme(scheme_name: &str, to_hash: ContentToHash, pwd_ref: &str) -> Result<()> {
    get_scheme(scheme_name)?.validate(&to_hash, pwd_ref)?;

    Ok(())
}

struct PwdParts {
    /// The scheme only (e.g., "#01")
    scheme_name: String,
    /// The Hashed password.
    hashed: String,
}

impl FromStr for PwdParts {
    type Err = Error;

    fn from_str(pwd_with_scheme: &str) -> Result<Self> {
        regex_captures!(r#"^#(\w+)#(.*)"#, pwd_with_scheme)
            .map(|(_, scheme, hashed)| Self {
                scheme_name: scheme.to_string(),
                hashed: hashed.to_string(),
            })
            .ok_or(Error::PwdWithSchemeFailedParse)
    }
}

// endregion:   --- Privates

// region:      --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_multi_scheme_ok() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::parse_str("c6410a5a-fca3-4d95-b6a9-623c26af79ce")?;
        let fx_to_hash = ContentToHash {
            content: "Hello world".to_string(),
            salt: fx_salt,
        };

        // -- Exec
        let pwd_hashed = hash_for_scheme("01", fx_to_hash.clone())?;
        println!("->>   pwd_hashed: {pwd_hashed}");
        let pwd_validate = validate_pwd(fx_to_hash.clone(), &pwd_hashed).await?;
        println!("->>   validate:   {pwd_validate:?}");

        // -- Check
        assert!(
            matches!(pwd_validate, SchemeStatus::Outdated),
            "status should be SchemaStatus::Outdated"
        );

        Ok(())
    }
}

// endregion:   --- Tests
