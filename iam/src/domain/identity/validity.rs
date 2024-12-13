use crate::domain::identity::Validity::{Between, OpenEnded, StartingOn, Until};
use anyhow::Result;
use chrono::{DateTime, Utc};
use common::validate;
use std::fmt::Display;
use std::ops::{RangeFrom, RangeInclusive, RangeToInclusive};

/// An enumeration value representing the validity of an invitation.
#[derive(Clone, Debug, PartialEq)]
pub enum Validity {
    /// A validity that is always valid.
    OpenEnded,
    /// A validity starting on specified date.
    StartingOn(RangeFrom<DateTime<Utc>>),
    /// A validity ending on specified date.
    Until(RangeToInclusive<DateTime<Utc>>),
    /// A validity valid between specified dates.
    Between(RangeInclusive<DateTime<Utc>>),
}

impl Validity {
    /// Create a new `Validity` instance with supplied start and end dates.
    ///
    /// It may return a [common::validate::Error] if both the start_date and end_date are provided
    /// and the start_date is later than the end_date.
    pub fn new(start_date: Option<DateTime<Utc>>, end_date: Option<DateTime<Utc>>) -> Result<Self> {
        match (start_date, end_date) {
            (Some(start_date), Some(end_date)) => {
                validate::max("start_date", &start_date, &end_date)?;
                Ok(Between(start_date..=end_date))
            }
            (Some(start_date), None) => Ok(StartingOn(start_date..)),
            (None, Some(end_date)) => Ok(Until(..=end_date)),
            (None, None) => Ok(OpenEnded),
        }
    }

    /// Retrieve the validity start date.
    pub fn start_date(&self) -> Option<&DateTime<Utc>> {
        match self {
            OpenEnded => None,
            StartingOn(range) => Some(&range.start),
            Until(_) => None,
            Between(range) => Some(range.start()),
        }
    }

    /// Retrieve the validity end date.
    pub fn end_date(&self) -> Option<&DateTime<Utc>> {
        match self {
            OpenEnded => None,
            StartingOn(_) => None,
            Until(range) => Some(&range.end),
            Between(range) => Some(range.end()),
        }
    }

    /// Check if the validity is valid now.
    pub fn is_valid(&self) -> bool {
        match self {
            OpenEnded => true,
            StartingOn(range) => range.contains(&Utc::now()),
            Until(range) => range.contains(&Utc::now()),
            Between(range) => range.contains(&Utc::now()),
        }
    }
}

impl Default for Validity {
    fn default() -> Self {
        OpenEnded
    }
}

impl Display for Validity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenEnded => write!(f, "open ended"),
            StartingOn(range) => write!(f, "from {}", range.start),
            Until(range) => write!(f, "to {}", range.end),
            Between(range) => {
                write!(f, "from {} to {}", range.start(), range.end())
            }
        }
    }
}