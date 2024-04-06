#![allow(unused)] // For early development.

// region:    --- Modules
mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;

// #[cfg(test)]
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;

use crate::model::ModelManager;
use crate::web::mw_auth::mw_ctx_require;
use crate::web::mw_auth::mw_ctx_resolve;
use crate::web::mw_res_map::mw_response_map;
use crate::web::rpc;
use crate::web::{routes_login, routes_static};
use axum::response::Html;
use axum::routing::get;
use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::{self, EnvFilter};
// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // For early local developement
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // FOR DEV ONLY
    _dev_utils::init_dev().await;

    // Initialize ModelManager.
    let mm = ModelManager::new().await?;

    // -- Define Routes
    let routes_rpc = rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

    let routes_all = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .nest("/api", routes_rpc)
        .layer(middleware::map_response(mw_response_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    // region:          --- Start Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    info!("{} - {:?}\n", "LISTENING", listener);
    axum::serve(listener, routes_all).await.unwrap();
    // endregion:       --- Start Server

    Ok(())
}
