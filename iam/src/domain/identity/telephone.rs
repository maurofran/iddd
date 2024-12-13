use std::fmt::Display;
use anyhow::Result;
use regex::Regex;
use common::validate;

#[derive(Debug, PartialEq, Clone)]
pub struct Telephone(String);

impl Telephone {
    pub fn new(telephone: &str) -> Result<Self> {
        const TELEPHONE: &str = "telephone";
        let pattern = Regex::new(r"((\(\d{3}\))|(\d{3}-))\d{3}-\d{4}")?;

        validate::not_empty(TELEPHONE, telephone)?;
        validate::length_between(TELEPHONE, telephone, 5, 20)?;
        validate::matches(TELEPHONE, telephone, pattern)?;
        Ok(Self(telephone.into()))
    }
}

impl Display for Telephone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Telephone {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl AsRef<str> for Telephone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}