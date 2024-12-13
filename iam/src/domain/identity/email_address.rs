use std::fmt::Display;
use anyhow::Result;
use regex::Regex;
use common::validate;

/// Represents an email address.
#[derive(Debug, Clone, PartialEq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email_address: &str) -> Result<Self> {
        const EMAIL_ADDRESS: &str = "email_address";
        let pattern = Regex::new(r"\w+([-+.']\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*")?;

        validate::not_empty(EMAIL_ADDRESS, email_address)?;
        validate::max_length(EMAIL_ADDRESS, email_address, 255)?;
        validate::matches(EMAIL_ADDRESS, email_address, pattern)?;
        Ok(Self(email_address.into()))
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}