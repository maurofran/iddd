use crate::domain::identity::Validity;
use chrono::Utc;
use uuid::Uuid;
use common::declare_simple_type;

declare_simple_type!(InvitationId, 36);
declare_simple_type!(InvitationDescription, 255);

impl InvitationId {
    /// Generates a new random `InvitationId`.
    pub fn random() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

/// Entity representing an invitation to register a tenant.
#[derive(Debug, Clone)]
pub struct RegistrationInvitation {
    invitation_id: InvitationId,
    description: InvitationDescription,
    validity: Validity,
}

impl RegistrationInvitation {
    /// Function used by repositories to hydrate [RegistrationInvitation] from the database.
    pub fn hydrate(
        invitation_id: InvitationId,
        description: InvitationDescription,
        validity: Validity,
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
            invitation_id: InvitationId::random(),
            description,
            validity: Validity::OpenEnded,
        }
    }

    /// Get the logical invitation unique identifier.
    pub fn invitation_id(&self) -> &InvitationId {
        &self.invitation_id
    }

    /// Get the description of the invitation.
    pub fn description(&self) -> &InvitationDescription {
        &self.description
    }

    /// Get the validity of the invitation.
    pub fn validity(&self) -> &Validity {
        &self.validity
    }

    /// Check if the invitation can be identified by the given identifier.
    pub fn is_identified_by(&self, identifier: &str) -> bool {
        self.invitation_id.as_ref() == identifier || self.description.as_ref() == identifier
    }

    /// Check if the invitation is available now.
    pub fn is_available(&self) -> bool {
        self.validity.is_valid()
    }

    /// Redefine the validity of the invitation with the provided value.
    pub fn redefine_as(&mut self, validity: Validity) {
        self.validity = validity;
    }
}

impl PartialEq for RegistrationInvitation {
    /// Two `RegistrationInvitation`s are equal if they have the same `invitation_id`.
    fn eq(&self, other: &Self) -> bool {
        self.invitation_id == other.invitation_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_hydrate() -> Result<()> {
        let invitation_id = InvitationId::new(&Uuid::new_v4().to_string())?;
        let description = InvitationDescription::new("a_description")?;
        let validity = Validity::OpenEnded;

        let fixture = RegistrationInvitation::hydrate(
            invitation_id.clone(),
            description.clone(),
            validity.clone(),
        );

        assert_eq!(fixture.invitation_id(), &invitation_id);
        assert_eq!(fixture.description(), &description);
        assert_eq!(fixture.validity(), &validity);
        Ok(())
    }

    #[test]
    pub fn test_new() -> Result<()> {
        let description = InvitationDescription::new("a_description")?;
        let fixture = RegistrationInvitation::new(description.clone());

        assert_eq!(fixture.description(), &description);
        assert_eq!(fixture.validity(), &Validity::OpenEnded);
        Ok(())
    }

    #[test]
    pub fn test_is_identified_by_id() -> Result<()> {
        let description = InvitationDescription::new("a_description")?;
        let fixture = RegistrationInvitation::new(description);

        assert!(fixture.is_identified_by(fixture.invitation_id()));
        Ok(())
    }

    #[test]
    pub fn test_is_identified_by_description() -> Result<()> {
        let description = InvitationDescription::new("a_description")?;
        let fixture = RegistrationInvitation::new(description);

        assert!(fixture.is_identified_by(fixture.description()));
        Ok(())
    }

    #[test]
    pub fn test_is_identified_by_not_identified() -> Result<()> {
        let description = InvitationDescription::new("a_description")?;
        let fixture = RegistrationInvitation::new(description);

        assert!(!fixture.is_identified_by("other"));
        Ok(())
    }

    #[test]
    pub fn test_is_available_open_ended() -> Result<()> {
        let description = InvitationDescription::new("a_description")?;
        let fixture = RegistrationInvitation::new(description);

        assert!(fixture.is_available());
        Ok(())
    }

    #[test]
    pub fn test_redefine_as() -> Result<()> {
        let description = InvitationDescription::new("a_description")?;
        let mut fixture = RegistrationInvitation::new(description);

        let new_validity =
            Validity::new(None, Some(Utc::now() + chrono::Duration::days(30))).unwrap();

        fixture.redefine_as(new_validity.clone());

        assert_eq!(fixture.validity(), &new_validity);
        Ok(())
    }
}
