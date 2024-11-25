use std::fmt::Display;
use chrono::{DateTime, Utc};
use anyhow::Result;
use common::validate;

const START_DATE: &str = "start_date";
const END_DATE: &str = "end_date";

/// InvitationValidity is the enum representing the validity of an invitation.
#[derive(Clone, Debug, PartialEq)]
pub enum InvitationValidity {
    // An always valid validity.
    OpenEnded,
    // A validity starting on specified date.
    StartingOn(DateTime<Utc>),
    // A validity ending on specified date.
    Until(DateTime<Utc>),
    // A validity valid between specified dates.
    Between(DateTime<Utc>, DateTime<Utc>),
}

impl InvitationValidity {
    /// Creates a new `InvitationValidity` with provided start and end dates.
    pub fn new(start_date: Option<DateTime<Utc>>, end_date: Option<DateTime<Utc>>) -> Result<Self> {
        match (start_date, end_date) {
            (Some(start), Some(end)) => {
                validate::max(START_DATE, &start, &end)?;
                Ok(InvitationValidity::Between(start, end))
            },
            (Some(start), None) => Ok(InvitationValidity::StartingOn(start)),
            (None, Some(end)) => Ok(InvitationValidity::Until(end)),
            (None, None) => Ok(InvitationValidity::OpenEnded),
        }
    }

    /// Factory function used to create an always valid invitation validity.
    pub fn open_ended() -> Self {
        InvitationValidity::OpenEnded
    }

    /// Creates a new `InvitationValidity` with provided starting date.
    pub fn starting_on(&self, date: DateTime<Utc>) -> Result<Self> {
        match self {
            InvitationValidity::OpenEnded | InvitationValidity::StartingOn(_) => Ok(InvitationValidity::StartingOn(date)),
            InvitationValidity::Until(end) | InvitationValidity::Between(_, end) => {
                validate::max(START_DATE, &date, &end)?;
                Ok(InvitationValidity::Between(date, end.clone()))
            }
        }
    }

    /// Creates a new `InvitationValidity` with provided ending date.
    pub fn until(&self, date: DateTime<Utc>) -> Result<Self> {
        match self {
            InvitationValidity::OpenEnded | InvitationValidity::Until(_) => Ok(InvitationValidity::Until(date)),
            InvitationValidity::StartingOn(start) | InvitationValidity::Between(start, _) => {
                validate::min(END_DATE, &date, &start)?;
                Ok(InvitationValidity::Between(start.clone(), date))
            }
        }
    }

    /// Getter for start date of the invitation.
    pub fn start_date(&self) -> Option<DateTime<Utc>> {
        match self {
            InvitationValidity::StartingOn(date) |
            InvitationValidity::Between(date, _) => Some(date.clone()),
            _ => None,
        }
    }

    /// Getter for end date of the invitation.
    pub fn end_date(&self) -> Option<DateTime<Utc>> {
        match self {
            InvitationValidity::Until(date) |
            InvitationValidity::Between(_, date) => Some(date.clone()),
            _ => None,
        }
    }

    /// Check if the invitation is available now.
    pub(super) fn is_available(&self) -> bool {
        match self {
            InvitationValidity::OpenEnded => true,
            InvitationValidity::StartingOn(date) => date <= &Utc::now(),
            InvitationValidity::Until(date) => date >= &Utc::now(),
            InvitationValidity::Between(start, end) => start <= &Utc::now() && end >= &Utc::now(),
        }
    }
}

impl Default for InvitationValidity {
    fn default() -> Self {
        InvitationValidity::OpenEnded
    }
}

impl Display for InvitationValidity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvitationValidity::OpenEnded => write!(f, "open ended"),
            InvitationValidity::StartingOn(date) => write!(f, "starting on {}", date),
            InvitationValidity::Until(date) => write!(f, "until {}", date),
            InvitationValidity::Between(start, end) => write!(f, "between {} and {}", start, end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_none_none() {
        let validity =  InvitationValidity::new(None, None).unwrap();
        assert_eq!(validity, InvitationValidity::OpenEnded);
    }

    #[test]
    fn new_some_none() {
        let start_date = Utc::now();
        let validity =  InvitationValidity::new(Some(start_date), None).unwrap();
        assert_eq!(validity, InvitationValidity::StartingOn(start_date));
    }

    #[test]
    fn new_none_some() {
        let end_date = Utc::now();
        let validity =  InvitationValidity::new(None, Some(end_date)).unwrap();
        assert_eq!(validity, InvitationValidity::Until(end_date));
    }

    #[test]
    fn new_some_some() {
        let start_date = Utc::now() - chrono::Duration::days(1);
        let end_date = Utc::now() + chrono::Duration::days(1);
        let validity = InvitationValidity::new(Some(start_date), Some(end_date)).unwrap();
        assert_eq!(validity, InvitationValidity::Between(start_date, end_date));
    }

    #[test]
    fn starting_on_success() {
        let start_date = Utc::now() - chrono::Duration::days(1);
        let validity = InvitationValidity::open_ended().starting_on(start_date).unwrap();
        assert_eq!(validity.start_date(), Some(start_date));
    }

    #[test]
    fn starting_on_fails() {
        let start_date = Utc::now() + chrono::Duration::days(2);
        let end_date = Utc::now() + chrono::Duration::days(1);
        let validity = InvitationValidity::open_ended()
            .until(end_date).unwrap()
            .starting_on(start_date).unwrap_err();

        assert_eq!(validity.downcast_ref::<validate::Error>().unwrap(),
                   &validate::Error::GreatestThan(START_DATE.into(), end_date.to_string()));
    }

    #[test]
    fn is_available_open_ended() {
        let validity = InvitationValidity::open_ended();
        assert!(validity.is_available());
    }

    #[test]
    fn is_available_starting_on_true() {
        let start_date = Utc::now() - chrono::Duration::days(1);
        let validity = InvitationValidity::open_ended().starting_on(start_date).unwrap();
        assert!(validity.is_available());
    }

    #[test]
    fn is_available_starting_on_false() {
        let start_date = Utc::now() + chrono::Duration::days(1);
        let validity = InvitationValidity::open_ended().starting_on(start_date).unwrap();
        assert!(!validity.is_available());
    }

    #[test]
    fn is_available_until_true() {
        let end_date = Utc::now() + chrono::Duration::days(1);
        let validity = InvitationValidity::open_ended().until(end_date).unwrap();
        assert!(validity.is_available());
    }

    #[test]
    fn is_available_until_false() {
        let end_date = Utc::now() - chrono::Duration::days(1);
        let validity = InvitationValidity::open_ended().until(end_date).unwrap();
        assert!(!validity.is_available());
    }

    #[test]
    fn is_available_between_true() {
        let start_date = Utc::now() - chrono::Duration::days(1);
        let end_date = Utc::now() + chrono::Duration::days(1);
        let validity = InvitationValidity::open_ended()
            .starting_on(start_date).unwrap()
            .until(end_date).unwrap();
        assert!(validity.is_available());
    }

    #[test]
    fn is_available_between_false_past() {
        let start_date = Utc::now() - chrono::Duration::days(2);
        let end_date = Utc::now() - chrono::Duration::days(1);
        let validity = InvitationValidity::open_ended()
            .starting_on(start_date).unwrap()
            .until(end_date).unwrap();
        assert!(!validity.is_available());
    }

    #[test]
    fn is_available_between_false_future() {
        let start_date = Utc::now() + chrono::Duration::days(1);
        let end_date = Utc::now() + chrono::Duration::days(2);
        let validity = InvitationValidity::open_ended()
            .starting_on(start_date).unwrap()
            .until(end_date).unwrap();
        assert!(!validity.is_available());
    }
}