use super::{Error, Result};
use crate::config::auth_config;
use crate::pwd::scheme::Scheme;
use argon2::password_hash::SaltString;
use argon2::{PasswordHash, PasswordHasher as _, PasswordVerifier as _};
use argon2::{Algorithm, Argon2, Params, Version};
use std::sync::OnceLock;

pub struct Scheme02;

impl Scheme for Scheme02 {
    fn hash(&self, to_hash: &crate::pwd::ContentToHash) -> Result<String> {
        let argon2 = get_argon2();

        let salt_b64 = SaltString::encode_b64(to_hash.salt.as_bytes()).map_err(|_| Error::Salt)?;

        let pwd = argon2
            .hash_password(to_hash.content.as_bytes(), &salt_b64)
            .map_err(|_| Error::Hash)?
            .to_string();

        Ok(pwd)
    }

    fn validate(&self, to_hash: &crate::pwd::ContentToHash, pwd_ref: &str) -> Result<()> {
        let argon2 = get_argon2();

        let parsed_hash_ref = PasswordHash::new(pwd_ref).map_err(|_| Error::Hash)?;

        argon2
            .verify_password(to_hash.content.as_bytes(), &parsed_hash_ref)
            .map_err(|_| Error::PwdValidate)?;

        Ok(())
    }
}

fn get_argon2() -> &'static Argon2<'static> {
    static INSTANCE: OnceLock<Argon2<'static>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let key = &auth_config().PWD_KEY;
        Argon2::new_with_secret(
            key,
            Algorithm::Argon2id, // Same as Argon2::default
            Version::V0x13,      // Same as Argon2::default
            Params::default(),
        )
        .unwrap() // TODO - needs early fail.
    })
}


// region:      --- Tests

#[cfg(test)]
mod tests {
    use crate::pwd::{scheme, ContentToHash};
    use super::*;
    use anyhow::Result;
    use uuid::Uuid;

    #[test]
    fn test_scheme_02_hash_into_b64u_ok() -> Result<()> {
        // -- Setup and fixtures
        let fx_to_hash = ContentToHash {
            content: "Hello world".to_string(),
            salt: Uuid::parse_str("c6410a5a-fca3-4d95-b6a9-623c26af79ce")?,
        };
        let fx_res = "$argon2id$v=19$m=19456,t=2,p=1$xkEKWvyjTZW2qWI8Jq95zg$PHjyVXh3qAyop+RXEYlT8mEHVaBJ0uB8fwPptJ3IQMc";

        // -- Exec
        let scheme2 = Scheme02;
        let res = scheme2.hash(&fx_to_hash)?;

        // -- Check
        assert_eq!(fx_res, res);

        Ok(())
    }
}

// endregion:   --- Tests