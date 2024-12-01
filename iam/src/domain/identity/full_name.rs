use common::constrained_string;
use std::fmt::Display;

constrained_string!(FirstName, 1, 50, r"[A-Z][a-z]*");
constrained_string!(LastName, 1, 50, r"^[a-zA-Z'][ a-zA-Z'-]*[a-zA-Z']?");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_name_new() {
        let first_name = FirstName::new("John").unwrap();
        let last_name = LastName::new("Doe").unwrap();
        let full_name = FullName::new(first_name, last_name);
        assert_eq!(full_name.to_string(), "John Doe");
    }

    #[test]
    fn test_full_name_as_formatted_name() {
        let first_name = FirstName::new("John").unwrap();
        let last_name = LastName::new("Doe").unwrap();
        let full_name = FullName::new(first_name, last_name);
        assert_eq!(full_name.as_formatted_name(), "John Doe");
    }
}