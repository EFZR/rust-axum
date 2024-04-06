//! Model Layer
//!
//! Design:
//!
//! - The Model Layer normalizes the application's data type
//!   Structures and access.
//! - All application code data access must go through the Model Layer.
//! - The `ModelManager` holds the internal states/resources
//!   needed by ModelControllers to access data.
//!   (e.g, db_pool, S3 client, redis client)
//! - Model Controllers (e.g, `TaskBmc`, `ProjectBmc`) implement
//!   CRUD and other data access methods on the given "entity".
//!   (e.g, `Task`, `Project`)
//!   (`Bmc` is short for backend Model Controller).
//! - Frameworks like Axum, Tauri, `ModelManager` are typically used as App State.
//!   ModelManager are designed to be passed as an argument
//!   To all Model Controllers function

// region:          ---modules

mod base;
mod error;
mod store;

pub mod task;
pub mod user;

pub use self::error::{Error, Result};

use self::store::{new_db_pool, Db};

// endregion:       ---modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    /// Returns the sqlx db pool reference.
    /// (ONLY for the model layer)
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
