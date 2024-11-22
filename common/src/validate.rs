use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("{0} is required")]
    Required(String),
    #[error("{0} must be different from {1}")]
    Equal(String, String),
    #[error("{0} must be equal to {1}")]
    NotEqual(String, String),
    #[error("{0} must be at least {1} characters long")]
    TooShort(String, usize),
    #[error("{0} must be long at most {1} characters")]
    TooLong(String, usize),
    #[error("{0} is greatest than {1}")]
    GreatestThan(String, String),
    #[error("{0} is lesser than {1}")]
    LesserThan(String, String),
    #[error("{0} must be between {1} and {2}")]
    NotInRange(String, String, String),
    #[error("{0} format is invalid")]
    InvalidFormat(String),
    #[error("{0}")]
    Generic(String),
}

/// Validates that a value is not empty.
pub fn not_empty(name: &str, value: &str) -> Result<(), Error> {
    if value.is_empty() {
        Err(Error::Required(name.into()))
    } else {
        Ok(())
    }
}

/// Check if a value is equal to a specific value.
pub fn equals<T: PartialEq + ToString>(name: &str, value: T, expected: T) -> Result<(), Error> {
    if value != expected {
        Err(Error::NotEqual(name.into(), expected.to_string()))
    } else {
        Ok(())
    }
}

pub fn not_equals<T: PartialEq + ToString>(name: &str, value: T, expected: T) -> Result<(), Error> {
    if value == expected {
        Err(Error::NotEqual(name.into(), expected.to_string()))
    } else {
        Ok(())
    }
}

pub fn is_true(value: bool, msg: &str) -> Result<(), Error> {
    if !value {
        Ok(())
    } else {
        Err(Error::Generic(msg.into()))
    }
}

pub fn is_false(value: bool, msg: &str) -> Result<(), Error> {
    if !value {
        Ok(())
    } else {
        Err(Error::Generic(msg.into()))
    }
}

/// Check if a value has a maximum length.
pub fn max_length(name: &str, value: &str, max: usize) -> Result<(), Error> {
    if value.len() > max {
        Err(Error::TooLong(name.into(), max))
    } else {
        Ok(())
    }
}

/// Check if a value has a minimum length.
pub fn min_length(name: &str, value: &str, min: usize) -> Result<(), Error> {
    if value.len() < min {
        Err(Error::TooShort(name.into(), min))
    } else {
        Ok(())
    }
}

/// Check that a value has a length between a minimum and maximum.
pub fn length_between(name: &str, value: &str, min: usize, max: usize) -> Result<(), Error> {
    min_length(name, value, min)?;
    max_length(name, value, max)?;
    Ok(())
}

pub fn max<T: PartialOrd + ToString>(name: &str, value: T, max: T) -> Result<(), Error> {
    if value > max {
        Err(Error::GreatestThan(name.into(), max.to_string()))
    } else {
        Ok(())
    }
}

pub fn min<T: PartialOrd + ToString>(name: &str, value: T, min: T) -> Result<(), Error> {
    if value < min {
        Err(Error::LesserThan(name.into(), min.to_string()))
    } else {
        Ok(())
    }
}

pub fn in_range<T: PartialOrd + ToString>(name: &str, value: T, min: T, max: T) -> Result<(), Error> {
    if value < min || value > max {
        Err(Error::NotInRange(name.into(), min.to_string(), max.to_string()))
    } else {
        Ok(())
    }
}

pub fn matches(name: &str, value: &str, regex: Regex) -> Result<(), Error> {
    if !regex.is_match(value) {
        Err(Error::InvalidFormat(name.into()))
    } else {
        Ok(())
    }
}