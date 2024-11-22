use thiserror::Error;
use crate::domain::identity::{Tenant, TenantId, TenantName, User, Username};
use anyhow::Result;

/// Error types for `UserRepository`.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserRepositoryError {
    #[error("no user found for tenant {0} and username '{1}'")]
    NotFound(TenantId, Username),
    #[error("a user with username '{1}' already exists in tenant {0}")]
    Exists(TenantId, Username)
}

pub trait UserRepository {
    fn add(&self, user: &User) -> Result<()>;
    fn update(&self, user: &User) -> Result<()>;
    fn remove(&self, user: &User) -> Result<()>;
    fn find_by_username(&self, tenant_id: &TenantId, username: &Username) -> Result<&User>;
    fn find_by_username_mut(&self, tenant_id: &TenantId, username: &Username) -> Result<&mut User>;
    fn find_all_similarly_named(&self, id: &TenantId, first_name_prefix: &str,
                                last_name_prefix: &str) -> Result<Vec<&Tenant>>;
}