use thiserror::Error;
use crate::domain::identity::{TenantId, User, Username};
use anyhow::Result;

/// Error types for `UserRepository`.
#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("no user found for tenant {0} and username '{1}'")]
    NotFound(TenantId, Username),
    #[error("a user with username '{1}' already exists in tenant {0}")]
    Exists(TenantId, Username)
}

pub trait UserRepository {
    async fn add(&self, user: &User) -> Result<()>;
    async fn update(&self, user: &User) -> Result<()>;
    async fn remove(&self, user: &User) -> Result<()>;
    async fn find_by_username(&self, tenant_id: &TenantId, username: &Username) -> Result<User>;
    async fn find_all_similarly_named(&self, id: &TenantId, first_name_prefix: &str,
                                      last_name_prefix: &str) -> Result<Vec<User>>;
}