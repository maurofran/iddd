use crate::domain::identity::InvitationValidity::{Between, OpenEnded, StartingOn, Until};
use chrono::{DateTime, Utc};
use std::fmt::Display;
use thiserror::Error;
use uuid::Uuid;
use crate::domain::identity::InvitationValidityError::{InvalidEndDate, InvalidStartDate};

/// Entity representing an invitation to register a tenant.
pub struct RegistrationInvitation {
    invitation_id: InvitationId,
    description: InvitationDescription,
    validity: InvitationValidity,
}

/// Function type used to redefine the validity of an invitation.
pub type InvitationRedefiner = fn(InvitationValidity) -> Result<InvitationValidity, InvitationValidityError>;

impl RegistrationInvitation {
    /// Function used by repositories to hydrate `RegistrationInvitation` from the database.
    /// Because of the use of the newtype patter, no invalid entity can be created.
    pub fn hydrate(invitation_id: InvitationId, description: InvitationDescription,
                   validity: InvitationValidity) -> Self {
        RegistrationInvitation {
            invitation_id,
            description,
            validity,
        }
    }

    /// Create a new registration invitation with the given description and default validity
    /// (open-ended).
    pub fn new(description: InvitationDescription) -> Self {
        Self {
            invitation_id: InvitationId::random(),
            description,
            validity: InvitationValidity::open_ended()
        }
    }

    /// Get the invitation id.
    pub fn invitation_id(&self) -> &InvitationId {
        &self.invitation_id
    }

    /// Get the description of the invitation.
    pub fn description(&self) -> &InvitationDescription {
        &self.description
    }

    /// Get the validity of the invitation.
    pub fn validity(&self) -> &InvitationValidity {
        &self.validity
    }

    /// Check if the invitation can be identified by the given identifier.
    pub fn is_identified_by(&self, identifier: &str) -> bool {
        self.invitation_id.as_ref() == identifier || self.description.as_ref() == identifier
    }

    pub fn is_available(&self) -> bool {
        self.validity.is_available()
    }

    /// Redefine the validity of the invitation with the provided closure function.
    pub fn redefine_as(&mut self, redefiner_fn: InvitationRedefiner) -> Result<(), InvitationValidityError> {
        self.validity = redefiner_fn(self.validity.clone())?;
        Ok(())
    }
}

/// InvitationId is a simple type for an invitation identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvitationId(String);

#[derive(Error, Clone, Debug, PartialEq)]
pub enum InvitationIdError {
    #[error("the invitation id is required")]
    Required,
    #[error("the invitation id must be 36 characters or less")]
    TooLong,
}

impl InvitationId {
    /// Generates a random `InvitationId`.
    pub fn random() -> Self {
        InvitationId(Uuid::new_v4().into())
    }

    /// Creates a new `InvitationId` from a string.
    pub fn new(id: &str) -> Result<Self, InvitationIdError> {
        if id.is_empty() {
            Err(InvitationIdError::Required)
        } else if id.len() > 36 {
            Err(InvitationIdError::TooLong)
        } else {
            Ok(InvitationId(id.into()))
        }
    }

    /// Returns the invitation id as a string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for InvitationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for InvitationId {
    type Error = InvitationIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        InvitationId::new(value)
    }
}

impl AsRef<str> for InvitationId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// InvitationDescription is a simple type for an invitation description.
#[derive(Debug, Clone, PartialEq)]
pub struct InvitationDescription(String);

/// Error for InvitationDescription.
#[derive(Error, Clone, Debug, PartialEq)]
pub enum InvitationDescriptionError {
    #[error("the invitation description is required")]
    Required,
    #[error("the invitation description must be 100 characters or less")]
    TooLong,
}

impl InvitationDescription {
    /// Creates a new `InvitationDescription` from a string.
    pub fn new(description: &str) -> Result<Self, InvitationDescriptionError> {
        if description.is_empty() {
            Err(InvitationDescriptionError::Required)
        } else if description.len() > 100 {
            Err(InvitationDescriptionError::TooLong)
        } else {
            Ok(InvitationDescription(description.into()))
        }
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for InvitationDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for InvitationDescription {
    type Error = InvitationDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        InvitationDescription::new(value)
    }
}

impl AsRef<str> for InvitationDescription {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

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

#[derive(Error, Clone, Debug, PartialEq)]
pub enum InvitationValidityError {
    #[error("the start date must occurs before {0}")]
    InvalidStartDate(DateTime<Utc>),
    #[error("the end date must occurs after {0}")]
    InvalidEndDate(DateTime<Utc>),
}

impl InvitationValidity {
    /// Factory function used to create an always valid invitation validity.
    pub fn open_ended() -> Self {
        OpenEnded
    }

    /// Creates a new `InvitationValidity` with provided starting date.
    pub fn starting_on(&self, date: DateTime<Utc>) -> Result<Self, InvitationValidityError> {
        match self {
            OpenEnded | StartingOn(_) => Ok(StartingOn(date)),
            Until(end) | Between(_, end) => if &date > end {
                Err(InvalidStartDate(end.clone()))
            } else {
                Ok(Between(date, end.clone()))
            }
        }
    }

    /// Creates a new `InvitationValidity` with provided ending date.
    pub fn until(&self, date: DateTime<Utc>) -> Result<Self, InvitationValidityError> {
        match self {
            OpenEnded | Until(_) => Ok(Until(date)),
            StartingOn(start) | Between(start, _) => if &date < start {
                Err(InvalidEndDate(start.clone()))
            } else {
                Ok(Between(start.clone(), date))
            }
        }
    }

    /// Check if the invitation is available now.
    fn is_available(&self) -> bool {
        match self {
            OpenEnded => true,
            StartingOn(date) => date <= &Utc::now(),
            Until(date) => date >= &Utc::now(),
            Between(start, end) => start <= &Utc::now() && end >= &Utc::now(),
        }
    }
}

impl Display for InvitationValidity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenEnded => write!(f, "open-ended"),
            StartingOn(date) => write!(f, "starting on {}", date),
            Until(date) => write!(f, "until {}", date),
            Between(start, end) => write!(f, "between {} and {}", start, end),
        }
    }
}

#[cfg(test)]
mod invitation_id_tests {
    use super::*;

    #[test]
    fn test_new_ok() {
        let id = InvitationId::new("test-id").unwrap();
        assert_eq!(id.into_string(), "test-id");
    }

    #[test]
    fn test_new_error_required() {
        let result = InvitationId::new("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), InvitationIdError::Required);
    }

    #[test]
    fn test_new_error_too_long() {
        let result = InvitationId::new("a".repeat(37).as_str());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), InvitationIdError::TooLong);
    }

    #[test]
    fn test_into_string() {
        let id = InvitationId::new("test-id").unwrap();
        assert_eq!(id.into_string(), "test-id");
    }
}