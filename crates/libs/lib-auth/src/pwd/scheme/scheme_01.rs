// region:     --- Modules

use super::{Error, Result};
use crate::auth_config;
use crate::pwd::scheme::Scheme;
use crate::pwd::ContentToHash;
use hmac::{Hmac, Mac};
use lib_utils::b64::b64u_encode;
use sha2::Sha512;

// endregion:  --- Modules

pub struct Scheme01;

impl Scheme for Scheme01 {
    fn hash(&self, to_hash: &ContentToHash) -> Result<String> {
        let key = &auth_config().PWD_KEY;
        hmac_sha512_hash(key, to_hash)
    }

    fn validate(&self, to_hash: &ContentToHash, pwd_ref: &str) -> Result<()> {
        let pwd = self.hash(to_hash)?;

        if pwd != pwd_ref {
            return Err(Error::PwdValidate);
        }

        Ok(())
    }
}

fn hmac_sha512_hash(key: &[u8], to_hash: &ContentToHash) -> Result<String> {
    let ContentToHash { content, salt } = to_hash;

    // -- Create HMAC-SHA-512 From key.
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::Key)?;

    // -- Add content.
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());

    // -- Finalize and b64u encode.
    let hmac_result = hmac_sha512.finalize();
    let result = b64u_encode(hmac_result.into_bytes());

    Ok(result)
}

// region:      --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use uuid::Uuid;

    #[test]
    fn test_scheme_hmac_into_b64u_ok() -> Result<()> {
        // -- Setup & Fixtures.
        let fx_salt = Uuid::parse_str("c6410a5a-fca3-4d95-b6a9-623c26af79ce")?;
        let fx_key = &auth_config().PWD_KEY; // 512 Bits = 64 bytes.
        let fx_to_hash = ContentToHash {
            content: "Hello world".to_string(),
            salt: fx_salt,
        };

        // -- TODO: Need to fix fx_key, and precompute fx_res.
        let fx_res = "P9QrbBQ7ni9qHPAVnVkt7Ea1PvwhTOSBSmSaRiWI4ku1AxGHdPgmwN7ju0MilsjcPnXID1vWxdYhwd5TxQx77A";

        // -- Exec
        let res = hmac_sha512_hash(&fx_key, &fx_to_hash)?;

        // -- Check 
        assert_eq!(res, fx_res);

        Ok(())
    }
}

// endregion:   --- Tests
