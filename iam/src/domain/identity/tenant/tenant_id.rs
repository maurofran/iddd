use anyhow::Result;
use derive_more::{AsRef, Deref, Display, From, Into};
use common::validate;
use uuid::Uuid;

const TENANT_ID: &str = "tenant_id";

/// A value object representing a unique tenant identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Deref, AsRef, From, Into)]
pub struct TenantId(Uuid);

impl TenantId {
    /// Creates a new random `TenantId`.
    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }

    /// Creates a new `TenantId` from a raw string.
    pub fn new(raw: &str) -> Result<Self> {
        let uuid = validate::uuid(TENANT_ID, raw)?;
        Ok(Self(uuid))
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
    fn test_into_uuid() {
        let tenant_id = &TenantId::new("123e4567-e89b-12d3-a456-426655440000").unwrap();
        let uuid: Uuid = tenant_id.clone().into();
        assert_eq!(
            uuid,
            Uuid::parse_str("123e4567-e89b-12d3-a456-426655440000").unwrap()
        );
    }
}
