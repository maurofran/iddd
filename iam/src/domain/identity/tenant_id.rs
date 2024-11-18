use std::fmt::Display;
use uuid::Uuid;
use thiserror::Error;

/// A value object representing a unique tenant identifier.
#[derive(Debug, Clone, PartialEq)]
pub struct TenantId(Uuid);

/// The error type for invalid tenant identifiers.
#[derive(Error, Debug, Clone, PartialEq)]
#[error("{0} is not a valid tenant identifier")]
pub struct TenantIdError(String);

impl TenantId {
    /// Creates a new random tenant identifier.
    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a new tenant identifier from a raw string.
    pub fn new(raw_tenant_id: &str) -> Result<Self, TenantIdError> {
        Uuid::parse_str(raw_tenant_id)
           .map(Self)
           .map_err(|_| TenantIdError(raw_tenant_id.into()))
    }

    /// Converts the tenant identifier to a UUID.
    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for TenantId {
    type Error = TenantIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TenantId::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tenant_id() {
        let tenant_id = TenantId::new("123e4567-e89b-12d3-a456-426655440000").unwrap();
        assert_eq!(tenant_id.to_string(), "123e4567-e89b-12d3-a456-426655440000");
    }

    #[test]
    fn test_invalid_tenant_id() {
        let result = TenantId::new("invalid-id");
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_str() {
        let tenant_id: TenantId = "123e4567-e89b-12d3-a456-426655440000".try_into().unwrap();
        assert_eq!(tenant_id.to_string(), "123e4567-e89b-12d3-a456-426655440000");
    }

    #[test]
    fn test_into_uuid() {
        let tenant_id = TenantId::new("123e4567-e89b-12d3-a456-426655440000").unwrap();
        let uuid = tenant_id.into_uuid();
        assert_eq!(uuid, Uuid::parse_str("123e4567-e89b-12d3-a456-426655440000").unwrap());
    }
}