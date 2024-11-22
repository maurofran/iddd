use std::fmt::Display;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct FullName {
    first_name: FirstName,
    last_name: LastName,
}

impl FullName {
    /// Creates a new `FullName` instance.
    pub fn new(first_name: FirstName, last_name: LastName) -> Self {
        Self {
            first_name,
            last_name,
        }
    }

    pub fn as_formatted_name(&self) -> String {
        self.to_string()
    }
}

impl Display for FullName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

fn first_name_regex() -> Regex {
    Regex::new(r"[A-Z][a-z]*").unwrap()
}

/// First name is the new type for the first name.
#[derive(Debug, Clone, PartialEq)]
pub struct FirstName(String);

impl FirstName {
    pub fn new(first_name: &str) -> Result<Self, FirstNameError> {
        if first_name.is_empty() {
            Err(FirstNameError::Required)
        } else if first_name.len() > 50 {
            Err(FirstNameError::TooLong)
        } else if !first_name_regex().is_match(&first_name) {
            Err(FirstNameError::InvalidFormat)
        } else {
            Ok(FirstName(first_name.into()))
        }
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for FirstName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for FirstName {
    type Error = FirstNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl AsRef<str> for FirstName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum FirstNameError {
    #[error("first name is required")]
    Required,
    #[error("first name must be 50 characters or less")]
    TooLong,
    #[error("first name must be at least one character in length, starting with a capital letter")]
    InvalidFormat,
}

fn last_name_regex() -> Regex {
    Regex::new(r"^[a-zA-Z'][ a-zA-Z'-]*[a-zA-Z']?").unwrap()
}

#[derive(Debug, Clone, PartialEq)]
pub struct LastName(String);

impl LastName {
    pub fn new(last_name: &str) -> Result<Self, LastNameError> {
        if last_name.is_empty() {
            Err(LastNameError::Required)
        } else if last_name.len() > 50 {
            Err(LastNameError::TooLong)
        } else if!last_name_regex().is_match(&last_name) {
            Err(LastNameError::InvalidFormat)
        } else {
            Ok(LastName(last_name.into()))
        }
    }
}

impl Display for LastName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for LastName {
    type Error = LastNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl AsRef<str> for LastName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum LastNameError {
    #[error("last name is required")]
    Required,
    #[error("last name must be 50 characters or less")]
    TooLong,
    #[error("last name must be at least one character in length")]
    InvalidFormat,
}