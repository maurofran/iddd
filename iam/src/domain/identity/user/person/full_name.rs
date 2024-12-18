use common::declare_simple_type;
use std::fmt::Display;

declare_simple_type!(FirstName, 50, r"^[A-Z][a-z]*$");
declare_simple_type!(LastName, 50, r"^[a-zA-Z'][ a-zA-Z'-]*[a-zA-Z']?");

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn first_name(&self) -> &FirstName {
        &self.first_name
    }

    pub fn last_name(&self) -> &LastName {
        &self.last_name
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
        let full_name = FullName::new("John".try_into().unwrap(), "Doe".try_into().unwrap());
        assert_eq!(full_name.to_string(), "John Doe");
    }

    #[test]
    fn test_full_name_as_formatted_name() {
        let full_name = FullName::new("John".try_into().unwrap(), "Doe".try_into().unwrap());
        assert_eq!(full_name.as_formatted_name(), "John Doe");
    }
}
