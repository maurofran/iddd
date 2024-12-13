use anyhow::Result;
use common::validate;
use regex::Regex;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    /// Creates a new `FullName` instance.
    pub fn new(first_name: &str, last_name: &str) -> Result<Self> {
        let mut full_name = Self {
            first_name: String::default(),
            last_name: String::default(),
        };
        full_name.set_first_name(first_name)?;
        full_name.set_last_name(last_name)?;
        Ok(full_name)
    }

    pub fn as_formatted_name(&self) -> String {
        self.to_string()
    }

    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    fn set_first_name(&mut self, first_name: &str) -> Result<()> {
        const FIRST_NAME: &str = "first_name";
        let pattern = Regex::new(r"^[A-Z][a-z]*$")?;

        validate::not_empty(FIRST_NAME, first_name)?;
        validate::max_length(FIRST_NAME, first_name, 50)?;
        validate::matches(FIRST_NAME, first_name, pattern)?;
        self.first_name = first_name.into();
        Ok(())
    }

    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    fn set_last_name(&mut self, last_name: &str) -> Result<()> {
        const LAST_NAME: &str = "last_name";
        let pattern = Regex::new(r"^[a-zA-Z'][ a-zA-Z'-]*[a-zA-Z']?")?;

        validate::not_empty(LAST_NAME, last_name)?;
        validate::max_length(LAST_NAME, last_name, 50)?;
        validate::matches(LAST_NAME, last_name, pattern)?;
        self.first_name = last_name.into();
        Ok(())
    }
}

impl Display for FullName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_name_new() {
        let full_name = FullName::new("John", "Doe").unwrap();
        assert_eq!(full_name.to_string(), "John Doe");
    }

    #[test]
    fn test_full_name_as_formatted_name() {
        let full_name = FullName::new("John", "Doe").unwrap();
        assert_eq!(full_name.as_formatted_name(), "John Doe");
    }
}
