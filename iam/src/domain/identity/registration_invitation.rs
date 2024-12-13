use crate::domain::identity::Validity;
use anyhow::Result;
use chrono::Utc;
use common::validate;
use uuid::Uuid;


fn validate_id(invitation_id: &str) -> Result<String> {
    const INVITATION_ID: &str = "invitation_id";

    validate::not_empty(INVITATION_ID, invitation_id)?;
    validate::max_length(INVITATION_ID, invitation_id, 36)?;
    Ok(invitation_id.into())
}

fn validate_description(description: &str) -> Result<String> {
    const DESCRIPTION: &str = "description";

    validate::not_empty(DESCRIPTION, description)?;
    validate::max_length(DESCRIPTION, description, 255)?;
    Ok(description.into())
}

/// Entity representing an invitation to register a tenant.
#[derive(Debug, Clone, PartialEq)]
pub struct RegistrationInvitation {
    id: Option<i32>,
    invitation_id: String,
    description: String,
    validity: Validity,
}

impl RegistrationInvitation {
    /// Function used by repositories to hydrate [RegistrationInvitation] from the database.
    pub fn hydrate(
        id: i32,
        invitation_id: &str,
        description: &str,
        validity: Validity,
    ) -> Result<Self> {
        Ok(RegistrationInvitation {
            id: Some(id),
            invitation_id: validate_id(invitation_id)?,
            description: validate_description(description)?,
            validity,
        })
    }

    /// Create a new registration invitation with the given description and default validity
    /// (open-ended).
    ///
    /// It may return a [common::validate::Error] if the description is empty or too long.
    pub fn new(description: &str) -> Result<Self> {
        Ok(Self {
            id: None,
            invitation_id: Uuid::new_v4().to_string(),
            description: validate_description(description)?,
            validity: Validity::OpenEnded,
        })
    }

    /// Get the unique identifier of the invitation.
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    /// Get the logical invitation unique identifier.
    pub fn invitation_id(&self) -> &str {
        &self.invitation_id
    }

    /// Get the description of the invitation.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the validity of the invitation.
    pub fn validity(&self) -> &Validity {
        &self.validity
    }

    /// Check if the invitation can be identified by the given identifier.
    pub fn is_identified_by(&self, identifier: &str) -> bool {
        self.invitation_id == identifier || self.description == identifier
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hydrate() {
        let id = 1;
        let invitation_id = Uuid::new_v4().to_string();
        let description = "a_description";
        let validity = Validity::OpenEnded;

        let fixture = RegistrationInvitation::hydrate(
            id,
            &invitation_id,
            description,
            validity.clone(),
        ).unwrap();

        assert_eq!(fixture.id(), Some(id));
        assert_eq!(fixture.invitation_id(), &invitation_id);
        assert_eq!(fixture.description(), description);
        assert_eq!(fixture.validity(), &validity);
    }

    #[test]
    pub fn test_new() {
        let description = "a_description";
        let fixture = RegistrationInvitation::new(description).unwrap();

        assert_eq!(fixture.id(), None);
        assert_eq!(fixture.description(), description);
        assert_eq!(fixture.validity(), &Validity::OpenEnded);
    }

    #[test]
    pub fn test_is_identified_by_id() {
        let description = "a_description";
        let fixture = RegistrationInvitation::new(description).unwrap();

        let identifier = fixture.invitation_id().as_ref();

        assert!(fixture.is_identified_by(identifier));
    }

    #[test]
    pub fn test_is_identified_by_description() {
        let description = "a_description";
        let fixture = RegistrationInvitation::new(description).unwrap();

        let identifier = fixture.description();

        assert!(fixture.is_identified_by(identifier));
    }

    #[test]
    pub fn test_is_identified_by_not_identified() {
        let description = "a_description";
        let fixture = RegistrationInvitation::new(description).unwrap();

        assert!(!fixture.is_identified_by("other"));
    }

    #[test]
    pub fn test_is_available_open_ended() {
        let description = "a_description";
        let fixture = RegistrationInvitation::new(description).unwrap();

        assert!(fixture.is_available());
    }

    #[test]
    pub fn test_redefine_as() {
        let description = "a_description";
        let mut fixture = RegistrationInvitation::new(description).unwrap();

        let new_validity = Validity::new(None, Some(Utc::now() + chrono::Duration::days(30))).unwrap();

        fixture.redefine_as(new_validity.clone());

        assert_eq!(fixture.validity(), &new_validity);
    }
}
