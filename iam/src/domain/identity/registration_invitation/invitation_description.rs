use std::fmt::Display;
use anyhow::Result;
use common::validate;

const DESCRIPTION: &str = "description";

/// InvitationDescription is a simple type for an invitation description.
#[derive(Debug, Clone, PartialEq)]
pub struct InvitationDescription(String);

impl InvitationDescription {
    /// Creates a new `InvitationDescription` from a string.
    pub fn new(description: &str) -> Result<Self> {
        validate::not_empty(DESCRIPTION, description)?;
        validate::max_length(DESCRIPTION, description, 100)?;
        Ok(InvitationDescription(description.into()))
    }
}

impl Display for InvitationDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for InvitationDescription {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        InvitationDescription::new(value)
    }
}

impl Into<String> for InvitationDescription {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for InvitationDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn new_success() {
        let description = InvitationDescription::new("a_description").unwrap();
        assert_eq!(description.to_string(), "a_description");
    }

    #[test]
    fn new_fails_with_required_error() {
        let result = InvitationDescription::new("");
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::Required(DESCRIPTION.into());
        assert_eq!(want, got);
    }

    #[test]
    fn new_fails_with_too_long_error() {
        let result = InvitationDescription::new("a".repeat(101).as_str());
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::TooLong(DESCRIPTION.into(), 100);
        assert_eq!(want, got);
    }

    #[test]
    fn to_string() {
        let fixture = InvitationDescription::new("a_description").unwrap();
        assert_eq!(fixture.to_string(), "a_description");
    }

    #[test]
    fn into_string() {
        let fixture = InvitationDescription::new("a_description").unwrap();
        let actual: String = fixture.clone().into();
        assert_eq!(actual, fixture.to_string());
    }

    #[test]
    fn try_from_succeed() {
        let description: InvitationDescription = "a_description".try_into().unwrap();
        assert_eq!(description.to_string(), "a_description");
    }

    #[test]
    fn try_from_fails_with_required_error() {
        let result: Result<InvitationDescription> = "".try_into();
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::Required(DESCRIPTION.into());
        assert_eq!(want, got);
    }

    #[test]
    fn try_from_fails_with_too_long_error() {
        let result: Result<InvitationDescription> = "a".repeat(101).as_str().try_into();
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::TooLong(DESCRIPTION.into(), 100);
        assert_eq!(want, got);
    }
}
