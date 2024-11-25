mod invitation_id;
mod invitation_description;
mod invitation_validity;

pub use invitation_description::InvitationDescription;
pub use invitation_id::InvitationId;
pub use invitation_validity::InvitationValidity;

use anyhow::Result;
use std::fmt::Display;

/// Entity representing an invitation to register a tenant.
pub struct RegistrationInvitation {
    invitation_id: InvitationId,
    description: InvitationDescription,
    validity: InvitationValidity,
}

/// Function type used to redefine the validity of an invitation.
pub type InvitationRedefiner = fn(InvitationValidity) -> Result<InvitationValidity>;

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
    pub fn redefine_as(&mut self, redefiner_fn: InvitationRedefiner) -> Result<()> {
        self.validity = redefiner_fn(self.validity.clone())?;
        Ok(())
    }
}