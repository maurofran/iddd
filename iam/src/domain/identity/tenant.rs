use crate::domain::identity::{InvitationDescription, InvitationDescriptor, InvitationValidity, RegistrationInvitation, TenantId};
use anyhow::Result;
use thiserror::Error;
use common::constrained_string;

constrained_string!(TenantName, 70);
constrained_string!(TenantDescription, 255);

// Tenant struct represent the aggregate root of the tenant domain.
pub struct Tenant {
    tenant_id: TenantId,
    name: TenantName,
    description: Option<TenantDescription>,
    status: TenantStatus,
    invitations: Vec<RegistrationInvitation>,
}

#[derive(Error, Debug, Clone, PartialEq)]
pub enum TenantError {
    #[error("tenant is not active")]
    NotActive,
    #[error("invitation with identifier {0} already exists")]
    InvitationExists(String),
    #[error("invitation with identifier {0} not found")]
    InvitationNotFound(String),
}

impl Tenant {
    /// Hydrates a new `Tenant` with the given values.
    pub fn hydrate(tenant_id: TenantId, name : TenantName, description: Option<TenantDescription>,
                   status: TenantStatus, invitations: Vec<RegistrationInvitation>) -> Self {
        Self {
            tenant_id,
            name,
            description,
            status,
            invitations,
        }
    }

    /// Creates a new `Tenant` with the given name, description, and status.
    pub fn new(name: TenantName, description: Option<TenantDescription>, status: TenantStatus) -> Self {
        Self {
            tenant_id: TenantId::random(),
            name,
            description,
            status,
            invitations: Vec::new(),
        }
    }

    /// Returns the unique identifier of the `Tenant`.
    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    /// Returns the name of the `Tenant`.
    pub fn name(&self) -> &TenantName {
        &self.name
    }

    /// Returns the optional description of the `Tenant`.
    pub fn description(&self) -> Option<&TenantDescription> {
        self.description.as_ref()
    }

    /// Returns the status of the `Tenant`.
    pub fn status(&self) -> &TenantStatus {
        &self.status
    }

    /// Returns the vector of all the invitations for the `Tenant`.
    pub fn invitations(&self) -> &[RegistrationInvitation] {
        &self.invitations
    }

    /// Activates the tenant.
    pub fn activate(&mut self) {
        self.status = TenantStatus::Active;
        // TODO Raise an event to indicate tenant activation
    }

    /// Deactivates the tenant.
    pub fn deactivate(&mut self) {
        self.status = TenantStatus::Inactive;
        // TODO Raise an event to indicate tenant deactivation
    }

    /// Retrieve a collection of all available registration invitations for the `Tenant`.
    pub fn all_available_registration_invitations(&self) -> Result<Vec<InvitationDescriptor>, TenantError> {
        self.assert_active()?;
        Ok(self.all_registration_invitations_for(true))
    }

    /// Retrieve a collection of all unavailable registration invitations for the `Tenant`.
    pub fn all_unavailable_registration_invitations(&self) -> Result<Vec<InvitationDescriptor>, TenantError> {
        self.assert_active()?;
        Ok(self.all_registration_invitations_for(false))
    }

    /// Check if a `Tenant` invitation is available through the provided identifier.
    pub fn is_registration_available_through(&self, identifier: &str) -> Result<bool, TenantError> {
        self.assert_active()?;
        match self.invitation(identifier) {
            Some(invitation) => Ok(invitation.is_available()),
            None => Ok(false),
        }
    }

    /// Offer a new registration invitation for the `Tenant`.
    pub fn offer_invitation(&mut self, description: InvitationDescription) -> Result<&mut RegistrationInvitation, TenantError> {
        self.assert_active()?;
        if self.is_registration_available_through(description.as_ref())? {
            return Err(TenantError::InvitationExists(description.into()));
        }
        let invitation = RegistrationInvitation::new(description.clone());
        self.invitations.push(invitation);
        match self.invitation_mut(description.as_ref()) {
            Some(invitation) => Ok(invitation),
            None => Err(TenantError::InvitationNotFound(description.into())),
        }
    }

    /// Redefine an existing registration invitation for the `Tenant`.
    pub fn redefine_invitation_as(&mut self, identifier: &str, validity: InvitationValidity) -> Result<()> {
        self.assert_active()?;
        if let Some(invitation) = self.invitation_mut(identifier) {
            invitation.redefine_as(validity);
            Ok(())
        } else {
            Err(TenantError::InvitationExists(identifier.to_string()).into())
        }
    }

    /// Withdraw an existing registration invitation for the `Tenant`.
    pub fn withdraw_invitation(&mut self, identifier: &str) -> Result<(), TenantError> {
        self.assert_active()?;
        if let Some(index) = self.invitations.iter().position(|invitation| invitation.is_identified_by(identifier)) {
            self.invitations.remove(index);
            Ok(())
        } else {
            Err(TenantError::InvitationNotFound(identifier.to_string()))
        }
    }

    fn all_registration_invitations_for(&self, available: bool) -> Vec<InvitationDescriptor> {
        self.invitations.iter()
            .filter(|invitation| invitation.is_available() == available)
            .map(|invitation| InvitationDescriptor::new(&self.tenant_id, invitation))
            .collect()
    }

    fn invitation(&self, identifier: &str) -> Option<&RegistrationInvitation> {
        self.invitations.iter()
            .find(|invitation| invitation.is_identified_by(identifier))
    }

    fn invitation_mut(&mut self, identifier: &str) -> Option<&mut RegistrationInvitation> {
        self.invitations.iter_mut()
           .find(|invitation| invitation.is_identified_by(identifier))
    }

    fn assert_active(&self) -> Result<(), TenantError> {
        match self.status {
            TenantStatus::Active => Ok(()),
            _ => Err(TenantError::NotActive),
        }
    }
}

/// The status of a `Tenant`.
#[derive(Debug, Clone, PartialEq)]
pub enum TenantStatus {
    Active,
    Inactive,
}

impl TenantStatus {
    /// Returns `true` if the tenant is active.
    pub fn is_active(&self) -> bool {
        match self {
            TenantStatus::Active => true,
            _ => false,
        }
    }
}