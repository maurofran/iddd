use common::validate;
use std::fmt::Display;

const USERNAME: &str = "username";

/// Value object representing a username.
#[derive(Debug, PartialEq, Clone)]
pub struct Username(String);

impl Username {
    /// Creates a new `Username` instance.
    pub fn new(username: &str) -> anyhow::Result<Self> {
        validate::not_empty(USERNAME, username)?;
        validate::max_length(USERNAME, username, 255)?;
        Ok(Username(username.into()))
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for Username {
    fn into(self) -> String {
        self.0
    }
}

impl TryFrom<&str> for Username {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        Username::new(value)
    }
}

impl AsRef<str> for Username {
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
        let username = Username::new("john_doe").unwrap();
        assert_eq!(username.to_string(), "john_doe");
    }

    #[test]
    fn new_fails_with_required_error() {
        let result = Username::new("");
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::Required(USERNAME.into());
        assert_eq!(want, got);
    }

    #[test]
    fn new_fails_with_too_long_error() {
        let result = Username::new("a".repeat(256).as_str());
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::TooLong(USERNAME.into(), 255);
        assert_eq!(want, got);
    }

    #[test]
    fn to_string() {
        let fixture = Username::new("john_doe").unwrap();
        assert_eq!(fixture.to_string(), "john_doe");
    }

    #[test]
    fn into_string() {
        let fixture = Username::new("john_doe").unwrap();
        let actual: String = fixture.clone().into();
        assert_eq!(actual, fixture.to_string());
    }

    #[test]
    fn try_from_succeed() {
        let username: Username = "john_doe".try_into().unwrap();
        assert_eq!(username.to_string(), "john_doe");
    }

    #[test]
    fn try_from_fails_with_required_error() {
        let result: Result<Username> = "".try_into();
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::Required(USERNAME.into());
        assert_eq!(want, got);
    }

    #[test]
    fn try_from_fails_with_too_long_error() {
        let result: Result<Username> = "a".repeat(256).as_str().try_into();
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::TooLong(USERNAME.into(), 255);
        assert_eq!(want, got);
    }
}
