use crate::rpcs::prelude::*;
use lib_core::model::project::{
    Project, ProjectBmc, ProjectFilter, ProjectForCreate, ProjectForUpdate,
};

pub fn rpc_router() -> RpcRouter {
    rpc_router!(create_project, get_project, list_projects, update_project, delete_project,)
}

generate_common_rpc_fns!(
    Bmc: ProjectBmc,
    Entity: Project,
    ForCreate: ProjectForCreate,
    ForUpdate: ProjectForUpdate,
    Filter: ProjectFilter,
    Suffix: project
);