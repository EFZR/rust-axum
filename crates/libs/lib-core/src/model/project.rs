// region:      --- Modules

use crate::ctx::Ctx;
use crate::generate_common_bmc_fns;
use crate::model::base::{self, DbBmc};
use crate::model::modql_utils::time_to_sea_value;
use crate::model::ModelManager;
use crate::model::Result;
use lib_utils::time::Rfc3339;
use modql::field::Fields;
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

// endregion:   --- Modules

// region:      --- Project Types

#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Project {
    pub id: i64,

    pub owner_id: i64,
    pub name: String,

    // -- Timestamps
    //    (creator and last modified user_id/time)
    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}

#[derive(Fields, Deserialize)]
pub struct ProjectForCreate {
    pub name: String,
}

#[derive(Fields, Deserialize)]
pub struct ProjectForUpdate {
    pub name: Option<String>,
    pub owner_id: Option<i64>,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct ProjectFilter {
    id: Option<OpValsInt64>,
    name: Option<OpValsString>,

    cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    ctime: Option<OpValsValue>,
    mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    mtime: Option<OpValsValue>,
}

// endregion:   --- Project Types

// region:      --- ProjectBmc

pub struct ProjectBmc;

impl DbBmc for ProjectBmc {
    const TABLE: &'static str = "project";

    fn has_owner_id() -> bool {
        true
    }
}

generate_common_bmc_fns!(
    Bmc: ProjectBmc,
    Entity: Project,
    ForCreate: ProjectForCreate,
    ForUpdate: ProjectForUpdate,
    Filter: ProjectFilter,
);

// endregion:   --- ProjectBmc
