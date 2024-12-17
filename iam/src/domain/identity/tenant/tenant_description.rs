use derive_more::{AsRef, Deref, Display};
use common::validate;
use anyhow::Result;

const DESCRIPTION: &str = "description";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Deref, AsRef, sqlx::Type)]
#[sqlx(transparent)]
pub struct TenantDescription(String);

impl TenantDescription {
    /// Creates a new `TenantDescription` instance from a given string.
    pub fn new(raw: &str) -> Result<Self> {
        validate::not_empty(DESCRIPTION, raw)?;
        validate::max_length(DESCRIPTION, raw, 255)?;
        Ok(Self(raw.into()))
    }

    /// Creates a new `TenantDescription` instance from a given string without validating it.
    pub unsafe fn new_unchecked(raw: &str) -> Self {
        Self(raw.into())
    }

    /// Converts the `TenantDescription` into a string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<&str> for TenantDescription {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<String> for TenantDescription {
    type Error = anyhow::Error;
    
    fn try_from(value: String) -> Result<Self> {
        Self::new(&value)
    }
}