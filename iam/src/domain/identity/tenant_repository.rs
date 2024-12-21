use thiserror::Error;
use anyhow::Result;
use crate::domain::identity::{Tenant, TenantId, TenantName};

/// Error types for `TenantRepository`.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TenantRepositoryError {
    #[error("no tenant found for {0}")]
    NotFound(TenantId),
    #[error("no tenant found for name '{0}'")]
    NameNotFound(TenantName),
    #[error("a tenant with name '{0}' already exists")]
    Exists(String)
}

/// A trait for repository pattern operations over [Tenant].
pub trait TenantRepository {
    /// Adds a new [Tenant] to the repository.
    /// It returns an [TenantRepositoryError::Exists] if a tenant with the same name already exists,
    /// or the representation of the stored tenant if successful.
    async fn add(&self, tenant: &Tenant) -> Result<()>;
    /// Updates an existing [Tenant] already in the repository.
    /// It returns an [TenantRepositoryError::NotFound] if no tenant with the given ID exists,
    /// [TenantRepositoryError::Exists] if a tenant with the same name already exists or the
    /// representation of the updated tenant if successful.
    async fn update(&self, tenant: &Tenant) -> Result<()>;
    /// Removes a [Tenant] from the repository.
    /// It returns an [TenantRepositoryError::NotFound] if no tenant with the given ID exists,
    async fn remove(&self, tenant: &Tenant) -> Result<()>;
    /// Finds a [Tenant] by its name.
    /// It returns an [TenantRepositoryError::NameNotFound] if no tenant with the given `name`
    /// exists in the storage.
    async fn find_by_name(&self, name: &TenantName) -> Result<Tenant>;
    /// Finds a [Tenant] by its unique identifier.
    /// It returns a [TenantRepositoryError::NotFound] if no tenant with the given ID exists in the
    /// storage.
    async fn find_by_id(&self, id: &TenantId) -> Result<Tenant>;
}