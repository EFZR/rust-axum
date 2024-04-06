// region:      --- Modules

mod error;
pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_login;
pub mod routes_static;
pub mod rpc;

use crate::crypt::token::generate_web_token;
use tower_cookies::{Cookie, Cookies};

pub use error::ClientError;
pub use error::{Error, Result};

// endregion:   --- Modules

pub const AUTH_TOKEN: &str = "auth-token";

pub fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

pub fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}
