// region:     Modules

mod config;
mod error;
mod log;
mod web;

pub use self::error::{Error, Result};
use config::web_config;

// endregion:  Modules

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}
