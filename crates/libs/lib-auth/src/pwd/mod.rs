// region:     Modules

mod error;
mod scheme;

pub use self::error::{Error, Result};
use self::scheme::{get_scheme, SchemeStatus, DEFAULT_SCHEME};
use lazy_regex::regex_captures;
use std::str::FromStr;
use uuid::Uuid;

// endregion:  Modules

// region:     Types

pub struct ContentToHash {
    pub content: String,
    pub salt: Uuid,
}

// endregion:  Types

// region:     Public Functions

/// Hash the password with the default scheme.
pub fn hash_pwd(to_hash: &ContentToHash) -> Result<String> {
    hash_for_scheme(DEFAULT_SCHEME, to_hash)
}

/// Validate if an ContentToHash matches.
pub fn validate_pwd(to_hash: &ContentToHash, pwd_ref: &str) -> Result<SchemeStatus> {
    let PwdParts {
        scheme_name,
        hashed,
    } = pwd_ref.parse()?;

    validate_for_scheme(&scheme_name, to_hash, &hashed)?;

    if scheme_name == DEFAULT_SCHEME {
        Ok(SchemeStatus::Ok)
    } else {
        Ok(SchemeStatus::Outdated)
    }
}

// endregion:  Public Functions

// region:      --- Privates

fn hash_for_scheme(scheme_name: &str, to_hash: &ContentToHash) -> Result<String> {
    let scheme = get_scheme(scheme_name)?;

    let pwd_hashed = scheme.hash(to_hash)?;

    Ok(format!("#{scheme_name}#{pwd_hashed}"))
}

fn validate_for_scheme(scheme_name: &str, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()> {
    get_scheme(scheme_name)?.validate(to_hash, pwd_ref)?;

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

    #[test]
    fn test_multi_scheme_ok() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::parse_str("c6410a5a-fca3-4d95-b6a9-623c26af79ce")?;
        let fx_to_hash = ContentToHash {
            content: "Hello world".to_string(),
            salt: fx_salt,
        };

        // -- Exec
        let pwd_hashed = hash_pwd(&fx_to_hash)?;
        println!("->>   pwd_hashed: {pwd_hashed}");
        let pwd_validate = validate_pwd(&fx_to_hash, &pwd_hashed)?;
        println!("->>   validate:   {pwd_validate:?}");

        Ok(())
    }
}

// endregion:   --- Tests
