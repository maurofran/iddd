use derive_more::{AsRef, Deref, Display};
use common::validate;
use anyhow::Result;

const NAME: &str = "name";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, Deref, AsRef, sqlx::Type)]
#[sqlx(transparent)]
pub struct TenantName(String);

impl TenantName {
    /// Creates a new `TenantName` instance from a given string.
    pub fn new(raw: &str) -> anyhow::Result<Self> {
        validate::not_empty(NAME, raw)?;
        validate::max_length(NAME, raw, 70)?;
        Ok(Self(raw.into()))
    }

    /// Creates a new `TenantName` instance from a given string without validating it.
    pub unsafe fn new_unchecked(raw: &str) -> Self {
        Self(raw.into())
    }

    /// Converts the `TenantName` into a string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<&str> for TenantName {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<String> for TenantName {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        Self::new(&value)
    }
}