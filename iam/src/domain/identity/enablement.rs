use std::fmt::Display;
use chrono::{DateTime, Utc};
use anyhow::Result;
use common::validate;

const START_DATE: &str = "start_date";

/// Value object representing an enablement status.
#[derive(Debug, PartialEq, Clone)]
pub enum Enablement {
    Enabled{
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    },
    Disabled,
}

impl Enablement {
    /// Creates a new `Enablement` instance.
    pub fn new(enabled: bool, start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Result<Self> {
        match (start, end) {
            (Some(start_date), Some(end_date)) => {
                validate::max(START_DATE, &start_date, &end_date)?;
            }
            _ => ()
        }
        let enablement = match enabled {
            true => Enablement::Enabled{start, end},
            false => Enablement::Disabled,
        };
        Ok(enablement)
    }

    /// Returns true if the enablement status is enabled.
    pub fn indefinite() -> Self {
        Enablement::Enabled{start: None, end: None}
    }

    /// Returns true if the enablement status is enabled.
    pub fn enabled(&self) -> bool {
        match self {
            Enablement::Enabled{..} => true,
            _ => false,
        }
    }

    /// Returns the start date, if any.
    pub fn start(&self) -> Option<&DateTime<Utc>> {
        match self {
            Enablement::Enabled{start,..} => start.as_ref(),
            _ => None,
        }
    }

    /// Returns the end date, if any.
    pub fn end(&self) -> Option<&DateTime<Utc>> {
        match self {
            Enablement::Enabled{end,..} => end.as_ref(),
            _ => None,
        }
    }

    pub fn is_time_expired(&self) -> bool {
        match self {
            Enablement::Enabled{start, end} => {
                let now = Utc::now();
                if let Some(start_date) = start {
                    return start_date > &now
                }
                if let Some(end_date) = end {
                    return end_date < &now
                }
                false
            }
            _ => false,
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            Enablement::Enabled {..} => !self.is_time_expired(),
            _ => false,  // Disabled status is always enabled
        }
    }
}

impl Display for Enablement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Enablement::Enabled { start, end } => {
                match (start, end) {
                    (Some(start_date), Some(end_date)) => {
                        write!(f, "Enabled from {} to {}", start_date, end_date)
                    }
                    (Some(start_date), None) => {
                        write!(f, "Enabled from {}", start_date)
                    }
                    (None, Some(end_date)) => {
                        write!(f, "Enabled until {}", end_date)
                    }
                    _ => write!(f, "Enabled"),
                }
            }
            Enablement::Disabled => {
                write!(f, "Disabled")
            }
        }
    }
}

impl From<bool> for Enablement {
    fn from(enabled: bool) -> Self {
        if enabled {
            Enablement::Enabled{start: None, end: None}
        } else {
            Enablement::Disabled
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn new_not_enabled() {
        let fixture = Enablement::new(false, None, None).unwrap();
        assert_eq!(fixture, Enablement::Disabled);
    }

    #[test]
    pub fn new_always_enabled() {
        let fixture = Enablement::new(true, None, None).unwrap();
        assert_eq!(fixture, Enablement::Enabled{start: None, end: None});
    }

    #[test]
    pub fn new_enabled_with_start_date() {
        let start_date = Utc::now();
        let fixture = Enablement::new(true, Some(start_date), None).unwrap();
        assert_eq!(fixture, Enablement::Enabled{start: Some(start_date), end: None});
    }

    #[test]
    pub fn new_enabled_with_end_date() {
        let end_date = Utc::now();
        let fixture = Enablement::new(true, None, Some(end_date)).unwrap();
        assert_eq!(fixture, Enablement::Enabled{start: None, end: Some(end_date)});
    }

    #[test]
    pub fn new_enabled_with_start_and_end_date() {
        let start_date = Utc::now();
        let end_date = start_date + chrono::Duration::days(30);
        let fixture = Enablement::new(true, Some(start_date), Some(end_date)).unwrap();
        assert_eq!(fixture, Enablement::Enabled{start: Some(start_date), end: Some(end_date)});
    }

    #[test]
    pub fn new_enabled_with_start_and_end_date_invalid() {
        let start_date = Utc::now();
        let end_date = start_date - chrono::Duration::days(30);
        let result = Enablement::new(true, Some(start_date), Some(end_date));
        assert!(result.is_err());
        let binding = result.unwrap_err();
        let got = binding.downcast_ref::<validate::Error>().unwrap();
        let want = &validate::Error::GreatestThan(START_DATE.into(), end_date.to_string());
        assert_eq!(got, want);
    }

    #[test]
    pub fn indefinite() {
        let fixture = Enablement::indefinite();
        assert_eq!(fixture, Enablement::Enabled{start: None, end: None});
    }

    #[test]
    pub fn enabled_true() {
        let fixture: Enablement = true.into();
        assert!(fixture.enabled());
    }

    #[test]
    pub fn enabled_false() {
        let fixture: Enablement = false.into();
        assert!(!fixture.enabled());
    }

    #[test]
    pub fn start_enabled() {
        let start_date = Utc::now();
        let fixture = Enablement::new(true, Some(start_date), None).unwrap();
        assert!(fixture.start().is_some());
        assert_eq!(fixture.start().unwrap(), &start_date);
    }

    #[test]
    pub fn start_disabled() {
        let fixture = Enablement::new(false, Some(Utc::now()), None).unwrap();
        assert!(fixture.start().is_none());
    }

    #[test]
    pub fn end_enabled() {
        let end_date = Utc::now();
        let fixture = Enablement::new(true, None, Some(end_date)).unwrap();
        assert!(fixture.end().is_some());
        assert_eq!(fixture.end().unwrap(), &end_date);
    }

    #[test]
    pub fn end_disabled() {
        let fixture = Enablement::new(false, None, Some(Utc::now())).unwrap();
        assert!(fixture.end().is_none());
    }

    #[test]
    pub fn is_time_expired_disabled() {
        let fixture = Enablement::new(false, None, None).unwrap();
        assert!(!fixture.is_time_expired());
    }

    #[test]
    pub fn is_time_expired_always_enabled() {
        let fixture = Enablement::new(true, None, None).unwrap();
        assert!(!fixture.is_time_expired());
    }

    #[test]
    pub fn is_time_expired_enabled_in_future() {
        let start_date = Utc::now() + chrono::Duration::days(30);
        let fixture = Enablement::new(true, Some(start_date), None).unwrap();
        assert!(fixture.is_time_expired());
    }

    #[test]
    pub fn is_time_expired_enabled_in_past() {
        let end_date = Utc::now() - chrono::Duration::days(30);
        let fixture = Enablement::new(true, None, Some(end_date)).unwrap();
        assert!(fixture.is_time_expired());
    }

    #[test]
    pub fn is_time_expired_actually_enabled() {
        let start_date = Utc::now() - chrono::Duration::days(30);
        let end_date = Utc::now() + chrono::Duration::days(30);
        let fixture = Enablement::new(true, Some(start_date), Some(end_date)).unwrap();
        assert!(!fixture.is_time_expired());
    }

    #[test]
    pub fn is_enabled_disabled() {
        let fixture = Enablement::new(false, None, None).unwrap();
        assert!(!fixture.is_enabled());
    }

    #[test]
    pub fn is_enabled_always_enabled() {
        let fixture = Enablement::new(true, None, None).unwrap();
        assert!(fixture.is_enabled());
    }

    #[test]
    pub fn is_enabled_enabled_in_future() {
        let start_date = Utc::now() + chrono::Duration::days(30);
        let fixture = Enablement::new(true, Some(start_date), None).unwrap();
        assert!(!fixture.is_enabled());
    }

    #[test]
    pub fn is_enabled_enabled_in_past() {
        let end_date = Utc::now() - chrono::Duration::days(30);
        let fixture = Enablement::new(true, None, Some(end_date)).unwrap();
        assert!(!fixture.is_enabled());
    }

    #[test]
    pub fn is_enabled_actually_enabled() {
        let start_date = Utc::now() - chrono::Duration::days(30);
        let end_date = Utc::now() + chrono::Duration::days(30);
        let fixture = Enablement::new(true, Some(start_date), Some(end_date)).unwrap();
        assert!(fixture.is_enabled());
    }

    #[test]
    pub fn true_into() {
        let fixture: Enablement = true.into();
        assert_eq!(fixture, Enablement::Enabled{start: None, end: None});
    }

    #[test]
    pub fn false_into() {
        let fixture: Enablement = false.into();
        assert_eq!(fixture, Enablement::Disabled);
    }
}