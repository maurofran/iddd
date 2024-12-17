use anyhow::Result;
use common::validate;
use derive_more::{AsRef, Deref, Display};

const INVITATION_DESCRIPTION: &str = "invitation_description";

/// Value object representing the description of an invitation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Deref, AsRef)]
pub struct InvitationDescription(String);

impl InvitationDescription {
    /// Creates a new `InvitationDescription` instance from a given string.
    pub fn new(raw: &str) -> Result<Self> {
        validate::not_empty(INVITATION_DESCRIPTION, raw)?;
        validate::max_length(INVITATION_DESCRIPTION, raw, 255)?;
        Ok(Self(raw.into()))
    }

    /// Creates a new `InvitationDescription` instance from a given string without validating it.
    pub unsafe fn new_unchecked(raw: &str) -> Self {
        Self(raw.into())
    }

    /// Converts the `InvitationDescription` into a string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<&str> for InvitationDescription {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<String> for InvitationDescription {
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
        let result = InvitationDescription::new(raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_fail_too_long() {
        let raw = "foo".repeat(100);
        let result = InvitationDescription::new(&raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_succeed() {
        let raw = "a description";
        let fixture = InvitationDescription::new(raw).unwrap();
        assert_eq!(fixture.0, raw);
    }

    #[test]
    fn test_to_string() {
        let raw = "a description";
        let fixture = InvitationDescription::new(raw).unwrap();
        assert_eq!(fixture.to_string(), raw);
    }

    #[test]
    fn test_try_from_str() {
        let raw = "a description";
        let fixture: InvitationDescription = raw.try_into().unwrap();
        assert_eq!(fixture.0, raw);
    }

    #[test]
    fn test_into_string() {
        let raw = "a description";
        let fixture = InvitationDescription::new(raw).unwrap();
        assert_eq!(fixture.into_string(), raw);
    }
}
