use anyhow::Result;
use common::validate;
use std::fmt::Display;
use uuid::Uuid;

const TENANT_ID: &str = "tenant_id";

/// A value object representing a unique tenant identifier.
#[derive(Debug, Clone, PartialEq)]
pub struct TenantId(Uuid);

impl TenantId {
    /// Creates a new random tenant identifier.
    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a new tenant identifier from a raw string.
    pub fn new(raw_tenant_id: &str) -> Result<Self> {
        let uuid = validate::uuid(TENANT_ID, raw_tenant_id)?;
        Ok(uuid.into())
    }
}

impl Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for TenantId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        TenantId::new(value)
    }
}

impl From<Uuid> for TenantId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<TenantId> for Uuid {
    fn from(tenant_id: TenantId) -> Self {
        tenant_id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tenant_id() {
        let tenant_id = TenantId::new("123e4567-e89b-12d3-a456-426655440000").unwrap();
        assert_eq!(
            tenant_id.to_string(),
            "123e4567-e89b-12d3-a456-426655440000"
        );
    }

    #[test]
    fn test_invalid_tenant_id() {
        let result = TenantId::new("invalid-id");
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_str() {
        let tenant_id: TenantId = "123e4567-e89b-12d3-a456-426655440000".try_into().unwrap();
        assert_eq!(
            tenant_id.to_string(),
            "123e4567-e89b-12d3-a456-426655440000"
        );
    }

    #[test]
    fn test_into_uuid() {
        let tenant_id = TenantId::new("123e4567-e89b-12d3-a456-426655440000").unwrap();
        let uuid: Uuid = tenant_id.into();
        assert_eq!(
            uuid,
            Uuid::parse_str("123e4567-e89b-12d3-a456-426655440000").unwrap()
        );
    }
}
