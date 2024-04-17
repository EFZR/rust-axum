//! Model Layer
//!
//! Design:
//!
//! - The Model layer normalizes the application's data type
//!   structures and access.
//! - All application code data access must go through the Model layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g., db_pool, S3 client, redis client).
//! - Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on a given "entity"
//!   (e.g., `Task`, `Project`).
//!   (`Bmc` is short for Backend Model Controller).
//! - In frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//! - ModelManager are designed to be passed as an argument
//!   to all Model Controllers functions.
//!

// region:     --- Modules

mod base;
mod error;
mod modql_utils;
mod store;

pub mod project;
pub mod task;
pub mod user;

pub use self::error::{Error, Result};

use crate::model::store::dbx::Dbx;
use crate::model::store::new_db_pool;

// endregion:  --- Modules

#[derive(Clone)]
pub struct ModelManager {
    dbx: Dbx,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        let dbx = Dbx::new(db, false)?;
        Ok(ModelManager { dbx })
    }

    /// Returns the sqlx db pool reference.
    /// (Only for the model layer)
    pub fn dbx(&self) -> &Dbx {
        &self.dbx
    }

    pub fn new_with_txn(&self) -> Result<ModelManager> {
        let dbx = Dbx::new(self.dbx.db().clone(), true)?;
        Ok(ModelManager { dbx })
    }
}
