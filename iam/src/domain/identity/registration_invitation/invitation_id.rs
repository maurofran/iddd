use anyhow::Result;
use common::validate;
use derive_more::{AsRef, Deref, Display};

const INVITATION_ID: &str = "invitation_id";

/// Value object representing a unique identifier for an invitation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Deref, AsRef)]
pub struct InvitationId(String);

impl InvitationId {
    /// Creates a new `InvitationId` instance from a given string.
    pub fn new(raw: &str) -> Result<Self> {
        validate::not_empty(INVITATION_ID, raw)?;
        validate::max_length(INVITATION_ID, raw, 36)?;
        Ok(Self(raw.into()))
    }

    /// Creates a new `InvitationId` instance from a given string without validating it.
    pub unsafe fn new_unchecked(raw: &str) -> Self {
        Self(raw.into())
    }

    /// Creates a new random `InvitationId` instance starting from a randomly generated UUID.
    pub fn random() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    /// Converts the `InvitationId` into a string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<&str> for InvitationId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<String> for InvitationId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        Self::new(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fail_empty() {
        let raw = "";
        let result = InvitationId::new(raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_fail_too_long() {
        let raw = "123e4567-e89b-12d3-a456-4266554400001";
        let result = InvitationId::new(raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_succeed() {
        let raw = "123e4567-e89b-12d3-a456-426655440000";
        let fixture = InvitationId::new(raw).unwrap();
        assert_eq!(fixture.0, raw);
    }

    #[test]
    fn test_try_from_str() {
        let raw = "123e4567-e89b-12d3-a456-426655440000";
        let fixture: InvitationId = raw.try_into().unwrap();
        assert_eq!(fixture.0, raw);
    }

    #[test]
    fn test_into_string() {
        let raw = "123e4567-e89b-12d3-a456-426655440000";
        let fixture = InvitationId::new(raw).unwrap();
        assert_eq!(fixture.into_string(), raw);
    }
}
