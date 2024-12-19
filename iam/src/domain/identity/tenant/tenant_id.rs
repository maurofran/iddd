use anyhow::Result;
use common::validate;
use derive_more::{AsRef, Deref, Display, From, Into};
use uuid::Uuid;

const TENANT_ID: &str = "tenant_id";

/// A value object representing a unique tenant identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Deref, AsRef)]
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

impl TryFrom<&str> for TenantId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl From<Uuid> for TenantId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl Into<Uuid> for TenantId {
    fn into(self) -> Uuid {
        (&self).0.clone()
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
}
