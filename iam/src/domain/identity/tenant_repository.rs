use thiserror::Error;
use crate::domain::identity::{Tenant, TenantId, TenantName};

/// Error types for `TenantRepository`.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TenantRepositoryError {
    #[error("no tenant found for {0}")]
    NotFound(TenantId),
    #[error("no tenant found for name '{1}'")]
    NameNotFound(TenantName),
    #[error("a tenant with name '{0}' already exists")]
    Exists(TenantName)
}

/// Result type for `TenantRepository`.
pub type Result<T> = std::result::Result<T, TenantRepositoryError>;

/// A trait for `TenantRepository` operations.
pub trait TenantRepository {
    fn add(&self, tenant: &Tenant) -> Result<()>;
    fn update(&self, tenant: &Tenant) -> Result<()>;
    fn remove(&self, tenant: &Tenant) -> Result<()>;
    fn find_by_name(&self, name: &TenantName) -> Result<&Tenant>;
    fn find_by_id(&self, id: &TenantId) -> Result<&Tenant>;
    /// Find a mutable reference to a tenant by its unique identifier.
    fn find_by_id_mut(&self, id: &TenantId) -> Result<&mut Tenant>;
}