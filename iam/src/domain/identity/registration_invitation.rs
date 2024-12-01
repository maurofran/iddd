use std::fmt::Display;
use std::ops::{RangeFrom, RangeInclusive, RangeToInclusive};

use chrono::{DateTime, Utc};
use common::constrained_string;
use uuid::Uuid;

constrained_string!(InvitationId, 1, 36);
constrained_string!(InvitationDescription, 1, 150);

/// Invitation validity is a range of dates.

/// Entity representing an invitation to register a tenant.
#[derive(Debug, Clone, PartialEq)]
pub struct RegistrationInvitation {
    invitation_id: InvitationId,
    description: InvitationDescription,
    validity: InvitationValidity,
}

impl RegistrationInvitation {
    /// Function used by repositories to hydrate `RegistrationInvitation` from the database.
    /// Because of the use of the new type patter, no invalid entity can be created.
    pub fn hydrate(
        invitation_id: InvitationId,
        description: InvitationDescription,
        validity: InvitationValidity,
    ) -> Self {
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
            invitation_id: InvitationId(Uuid::new_v4().into()),
            description,
            validity: InvitationValidity::OpenEnded,
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

    /// Check if the invitation is available now.
    pub fn is_available(&self) -> bool {
        self.validity.is_available()
    }

    /// Redefine the validity of the invitation with the provided closure function.
    pub fn redefine_as(&mut self, validity: InvitationValidity) {
        self.validity = validity;
    }
}

/// InvitationValidity is the enum representing the validity of an invitation.
#[derive(Clone, Debug, PartialEq)]
pub enum InvitationValidity {
    // An always valid validity.
    OpenEnded,
    // A validity starting on specified date.
    StartingOn(RangeFrom<DateTime<Utc>>),
    // A validity ending on specified date.
    Until(RangeToInclusive<DateTime<Utc>>),
    // A validity valid between specified dates.
    Between(RangeInclusive<DateTime<Utc>>),
}

impl InvitationValidity {
    /// Getter for start date of the invitation.
    pub fn start_date(&self) -> Option<&DateTime<Utc>> {
        match self {
            InvitationValidity::OpenEnded => None,
            InvitationValidity::StartingOn(range) => Some(&range.start),
            InvitationValidity::Until(_) => None,
            InvitationValidity::Between(range) => Some(range.start()),
        }
    }

    /// Getter for end date of the invitation.
    pub fn end_date(&self) -> Option<&DateTime<Utc>> {
        match self {
            InvitationValidity::OpenEnded => None,
            InvitationValidity::StartingOn(_) => None,
            InvitationValidity::Until(range) => Some(&range.end),
            InvitationValidity::Between(range) => Some(range.end()),
        }
    }

    /// Check if the invitation is available now.
    fn is_available(&self) -> bool {
        match self {
            InvitationValidity::OpenEnded => true,
            InvitationValidity::StartingOn(range) => range.contains(&Utc::now()),
            InvitationValidity::Until(range) => range.contains(&Utc::now()),
            InvitationValidity::Between(range) => range.contains(&Utc::now()),
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
            InvitationValidity::StartingOn(range) => write!(f, "from {}", range.start),
            InvitationValidity::Until(range) => write!(f, "to {}", range.end),
            InvitationValidity::Between(range) => {
                write!(f, "from {} to {}", range.start(), range.end())
            }
        }
    }
}

impl From<RangeFrom<DateTime<Utc>>> for InvitationValidity {
    fn from(range: RangeFrom<DateTime<Utc>>) -> Self {
        InvitationValidity::StartingOn(range)
    }
}

impl From<RangeToInclusive<DateTime<Utc>>> for InvitationValidity {
    fn from(range: RangeToInclusive<DateTime<Utc>>) -> Self {
        InvitationValidity::Until(range)
    }
}

impl From<RangeInclusive<DateTime<Utc>>> for InvitationValidity {
    fn from(range: RangeInclusive<DateTime<Utc>>) -> Self {
        InvitationValidity::Between(range)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydrate() {
        let invitation_id = InvitationId::new(Uuid::new_v4().to_string().as_str()).unwrap();
        let description = InvitationDescription::new("a_description").unwrap();
        let validity = InvitationValidity::OpenEnded;

        let fixture = RegistrationInvitation::hydrate(
            invitation_id.clone(),
            description.clone(),
            validity.clone(),
        );

        assert_eq!(fixture.invitation_id(), &invitation_id);
        assert_eq!(fixture.description(), &description);
        assert_eq!(fixture.validity(), &InvitationValidity::OpenEnded);
    }

    #[test]
    pub fn test_new() {
        let description = InvitationDescription::new("a_description").unwrap();
        let fixture = RegistrationInvitation::new(description.clone());

        assert_eq!(fixture.description(), &description);
        assert_eq!(fixture.validity(), &InvitationValidity::OpenEnded);
    }

    #[test]
    pub fn test_is_identified_by_id() {
        let description = InvitationDescription::new("a_description").unwrap();
        let fixture = RegistrationInvitation::new(description);

        let identifier = fixture.invitation_id().as_ref();

        assert!(fixture.is_identified_by(identifier));
    }

    #[test]
    pub fn test_is_identified_by_description() {
        let description = InvitationDescription::new("a_description").unwrap();
        let fixture = RegistrationInvitation::new(description);

        let identifier = fixture.description().as_ref();

        assert!(fixture.is_identified_by(identifier));
    }

    #[test]
    pub fn test_is_identified_by_not_identified() {
        let description = InvitationDescription::new("a_description").unwrap();
        let fixture = RegistrationInvitation::new(description);

        assert!(!fixture.is_identified_by("other"));
    }

    #[test]
    pub fn test_is_available_open_ended() {
        let description = InvitationDescription::new("a_description").unwrap();
        let fixture = RegistrationInvitation::new(description);

        assert!(fixture.is_available());
    }

    #[test]
    pub fn test_redefine_as() {
        let description = InvitationDescription::new("a_description").unwrap();
        let mut fixture = RegistrationInvitation::new(description);

        let new_validity: InvitationValidity = (..=Utc::now() + chrono::Duration::days(30)).into();

        fixture.redefine_as(new_validity.clone());

        assert_eq!(fixture.validity(), &new_validity);
    }
}
