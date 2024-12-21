use crate::domain::identity::Validity;
use chrono::Utc;
use std::fmt::Display;

/// Value object representing an enablement status.
#[derive(Debug, PartialEq, Clone)]
pub enum Enablement {
    Enabled(Validity),
    Disabled,
}

impl Enablement {
    /// Creates a new [Enablement] instance.
    pub fn new(enabled: bool, validity: Validity) -> Self {
        match enabled {
            true => Enablement::Enabled(validity),
            false => Enablement::Disabled,
        }
    }

    /// Creates an enabled [Enablement] with indefinite validity.
    pub fn indefinite() -> Self {
        Enablement::Enabled(Validity::OpenEnded)
    }

    /// Returns true if the enablement status is enabled.
    pub fn enabled(&self) -> bool {
        match self {
            Enablement::Enabled(_) => true,
            _ => false,
        }
    }

    /// Returns the start date, if any.
    pub fn validty(&self) -> Option<&Validity> {
        match self {
            Enablement::Enabled(validity) => Some(validity),
            _ => None,
        }
    }

    pub fn is_time_expired(&self) -> bool {
        match self {
            Enablement::Enabled(validity) => !validity.is_valid(),
            _ => false,
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            Enablement::Enabled(_) => !self.is_time_expired(),
            _ => false, // Disabled status is always disabled
        }
    }
}

impl Display for Enablement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Enablement::Enabled(validity) => write!(f, "Enabled {}", validity),
            Enablement::Disabled => write!(f, "Disabled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_not_enabled() {
        let fixture = Enablement::new(false, Validity::OpenEnded);
        assert_eq!(fixture, Enablement::Disabled);
    }

    #[test]
    pub fn new_always_enabled() {
        let fixture = Enablement::new(true, Validity::OpenEnded);
        assert_eq!(fixture, Enablement::Enabled(Validity::OpenEnded));
    }

    #[test]
    pub fn new_enabled_with_start_date() {
        let start_date = Utc::now();
        let validity = Validity::new(Some(start_date), None).unwrap();
        let fixture = Enablement::new(true, validity.clone());
        assert_eq!(fixture, Enablement::Enabled(validity));
    }

    #[test]
    pub fn new_enabled_with_end_date() {
        let end_date = Utc::now();
        let validity = Validity::new(None, Some(end_date)).unwrap();
        let fixture = Enablement::new(true, validity.clone());
        assert_eq!(fixture, Enablement::Enabled(validity));
    }

    #[test]
    pub fn new_enabled_with_start_and_end_date() {
        let start_date = Utc::now();
        let end_date = start_date + chrono::Duration::days(30);
        let validity = Validity::new(Some(start_date), Some(end_date)).unwrap();
        let fixture = Enablement::new(true, validity.clone());
        assert_eq!(fixture, Enablement::Enabled(validity));
    }

    #[test]
    pub fn indefinite() {
        let fixture = Enablement::indefinite();
        assert_eq!(fixture, Enablement::Enabled(Validity::OpenEnded));
    }
}
