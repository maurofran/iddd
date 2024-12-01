use crate::domain::access::{Role, RoleName};
use crate::domain::identity::TenantId;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum RoleRepositoryError {
    #[error("no role found for tenant {0} and name '{1}'")]
    NotFound(TenantId, RoleName),
    #[error("a role with name '{1}' already exists in tenant {0}")]
    Exists(TenantId, RoleName),
}

pub trait RoleRepository {
    fn add(&self, role: &Role) -> Result<()>;
    fn update(&self, role: &Role) -> Result<()>;
    fn remove(&self, role: &Role) -> Result<()>;
    fn find_by_name(&self, tenant_id: &TenantId, name: &RoleName) -> Result<&Role>;
    fn find_by_name_mut(&self, tenant_id: &TenantId, name: &RoleName) -> Result<&mut Role>;
    fn find_all(&self, id: &TenantId) -> Result<Vec<&Role>>;
}
