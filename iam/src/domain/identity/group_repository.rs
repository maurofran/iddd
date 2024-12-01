use crate::domain::identity::{Group, GroupName, TenantId};
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum GroupRepositoryError {
    #[error("no group found for tenant {0} and name '{1}'")]
    NotFound(TenantId, GroupName),
    #[error("a group with name '{1}' already exists in tenant {0}")]
    Exists(TenantId, GroupName),
}

pub trait GroupRepository {
    fn add(&self, group: &Group) -> Result<()>;
    fn update(&self, group: &Group) -> Result<()>;
    fn remove(&self, group: &Group) -> Result<()>;
    fn find_by_name(&self, tenant_id: &TenantId, name: &GroupName) -> Result<&Group>;
    fn find_by_name_mut(&self, tenant_id: &TenantId, name: &GroupName) -> Result<&mut Group>;
    fn find_all(&self, id: &TenantId) -> Result<Vec<&Group>>;
}
