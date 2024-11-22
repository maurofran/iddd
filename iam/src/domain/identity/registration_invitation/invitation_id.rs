use std::fmt::Display;
use anyhow::Result;
use uuid::Uuid;
use common::validate;

const INVITATION_ID: &str = "invitation id";

/// InvitationId is a simple type for an invitation identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvitationId(String);

impl InvitationId {
    /// Generates a random `InvitationId`.
    pub fn random() -> Self {
        InvitationId(Uuid::new_v4().into())
    }

    /// Creates a new `InvitationId` from a string.
    pub fn new(id: &str) -> Result<Self> {
        validate::not_empty(INVITATION_ID, id)?;
        validate::max_length(INVITATION_ID, id, 255)?;
        Ok(InvitationId(id.into()))
    }
}

impl Display for InvitationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for InvitationId {
    fn into(self) -> String {
        self.0
    }
}

impl TryFrom<&str> for InvitationId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        InvitationId::new(value)
    }
}

impl AsRef<str> for InvitationId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn new_success() {
        let invitation_id = InvitationId::new("123e4567-e89b-12d3-a456-426655440000").unwrap();
        assert_eq!(invitation_id.to_string(), "123e4567-e89b-12d3-a456-426655440000");
    }

    #[test]
    fn new_fails_with_required_error() {
        let result = InvitationId::new("");
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::Required(INVITATION_ID.into());
        assert_eq!(want, got);
    }

    #[test]
    fn new_fails_with_too_long_error() {
        let result = InvitationId::new("a".repeat(256).as_str());
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::TooLong(INVITATION_ID.into(), 255);
        assert_eq!(want, got);
    }

    #[test]
    fn to_string() {
        let fixture = InvitationId::new("123e4567-e89b-12d3-a456-426655440000").unwrap();
        assert_eq!(fixture.to_string(), "123e4567-e89b-12d3-a456-426655440000");
    }

    #[test]
    fn into_string() {
        let fixture = InvitationId::new("123e4567-e89b-12d3-a456-426655440000").unwrap();
        let actual: String = fixture.clone().into();
        assert_eq!(actual, "123e4567-e89b-12d3-a456-426655440000".to_string());
    }

    #[test]
    fn try_from_succeed() {
        let invitation_id: InvitationId = "123e4567-e89b-12d3-a456-426655440000".try_into().unwrap();
        let actual: String = invitation_id.into();
        assert_eq!(actual, "123e4567-e89b-12d3-a456-426655440000".to_string());
    }

    #[test]
    fn try_from_fails_with_required_error() {
        let result: Result<InvitationId> = "".try_into();
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::Required(INVITATION_ID.into());
        assert_eq!(want, got);
    }

    #[test]
    fn try_from_fails_with_too_long_error() {
        let result: Result<InvitationId> = "a".repeat(256).as_str().try_into();
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::TooLong(INVITATION_ID.into(), 255);
        assert_eq!(want, got);
    }
}