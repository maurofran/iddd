mod tenant_id;

use crate::domain::identity::{
    InvitationDescription, InvitationDescriptor, RegistrationInvitation, Validity,
};
use anyhow::{bail, Result};
use common::declare_simple_type;
pub use tenant_id::TenantId;
use thiserror::Error;

declare_simple_type!(TenantName, 70);
declare_simple_type!(TenantDescription, 255);

/// Value object representing the [Tenant] possible error conditions.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TenantError {
    #[error("tenant is not active")]
    NotActive,
    #[error("invitation with identifier {0} already exists")]
    InvitationExists(String),
    #[error("invitation with identifier {0} not found")]
    InvitationNotFound(String),
}

/// Tenant struct represent the aggregate root of the tenant domain.
#[derive(Debug)]
pub struct Tenant {
    tenant_id: TenantId,
    name: TenantName,
    description: Option<TenantDescription>,
    active: bool,
    invitations: Vec<RegistrationInvitation>,
}

impl Tenant {
    /// Hydrates an existing [Tenant] from the persistent storage.
    pub fn hydrate(
        tenant_id: TenantId,
        name: TenantName,
        description: Option<TenantDescription>,
        active: bool,
        invitations: Vec<RegistrationInvitation>,
    ) -> Self {
        Self {
            tenant_id,
            name,
            description,
            active,
            invitations,
        }
    }

    /// Creates a new [Tenant] with the given name, description, and status.
    pub fn new(name: TenantName, description: Option<TenantDescription>, active: bool) -> Self {
        Self {
            tenant_id: TenantId::random(),
            name,
            description,
            active,
            invitations: Vec::new(),
        }
    }

    /// Returns the logical unique identifier of the [Tenant]. This field is a randomly generated
    /// UUID and is assigned upon [Tenant] creation.
    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    /// Returns the name of the [Tenant].
    pub fn name(&self) -> &TenantName {
        &self.name
    }

    /// Returns the optional description of the [Tenant].
    pub fn description(&self) -> &Option<TenantDescription> {
        &self.description
    }

    /// Returns `true` if the [Tenant] is active.
    pub fn active(&self) -> bool {
        self.active
    }

    /// Returns an array of all the [RegistrationInvitation] associated with the [Tenant].
    pub fn invitations(&self) -> &[RegistrationInvitation] {
        &self.invitations
    }

    /// Activates the [Tenant].
    pub fn activate(&mut self) {
        self.active = true;
        // TODO Raise an event to indicate tenant activation
    }

    /// Deactivates the [Tenant].
    pub fn deactivate(&mut self) {
        self.active = false;
        // TODO Raise an event to indicate tenant deactivation
    }

    /// Retrieve a collection of all available registration invitations for the [Tenant] as
    /// [InvitationDescriptor].
    ///
    /// It may return a [TenantError::NotActive] if the [Tenant] is not active.
    pub fn all_available_registration_invitations(&self) -> Result<Vec<InvitationDescriptor>> {
        self.assert_active()?;
        Ok(self.all_registration_invitations_for(true))
    }

    /// Retrieve a collection of all unavailable registration invitations for the [Tenant] as
    /// [InvitationDescriptor].
    ///
    /// It may return a [TenantError::NotActive] if the [Tenant] is not active.
    pub fn all_unavailable_registration_invitations(&self) -> Result<Vec<InvitationDescriptor>> {
        self.assert_active()?;
        Ok(self.all_registration_invitations_for(false))
    }

    /// Check if a [Tenant] invitation is available through the provided identifier.
    ///
    /// It may return a [TenantError::NotActive] if the [Tenant] is not active.
    pub fn is_registration_available_through(&self, identifier: &str) -> Result<bool> {
        self.assert_active()?;
        match self.invitation(identifier) {
            Some(invitation) => Ok(invitation.is_available()),
            None => Ok(false),
        }
    }

    /// Offer a new registration invitation for the [Tenant].
    ///
    /// It may return a [TenantError::NotActive] if the [Tenant] is not active or a
    /// [TenantError::InvitationExists] if an invitation with the same description already exists.
    pub fn offer_invitation(&mut self, description: &str) -> Result<&mut RegistrationInvitation> {
        self.assert_active()?;
        if self.is_registration_available_through(description)? {
            bail!(TenantError::InvitationExists(description.into()));
        }
        let invitation_description = InvitationDescription::new(description)?;
        let invitation = RegistrationInvitation::new(invitation_description);
        self.invitations.push(invitation);
        match self.invitation_mut(description.as_ref()) {
            Some(invitation) => Ok(invitation),
            None => bail!(TenantError::InvitationNotFound(description.into())),
        }
    }

    /// Redefine an existing registration invitation for the `Tenant`.
    pub fn redefine_invitation_as(&mut self, identifier: &str, validity: Validity) -> Result<()> {
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
        if let Some(index) = self
            .invitations
            .iter()
            .position(|invitation| invitation.is_identified_by(identifier))
        {
            self.invitations.remove(index);
            Ok(())
        } else {
            Err(TenantError::InvitationNotFound(identifier.to_string()))
        }
    }

    fn all_registration_invitations_for(&self, available: bool) -> Vec<InvitationDescriptor> {
        self.invitations
            .iter()
            .filter(|invitation| invitation.is_available() == available)
            .map(|invitation| InvitationDescriptor::new(&self.tenant_id, invitation))
            .collect::<Vec<_>>()
    }

    fn invitation(&self, identifier: &str) -> Option<&RegistrationInvitation> {
        self.invitations
            .iter()
            .find(|invitation| invitation.is_identified_by(identifier))
    }

    fn invitation_mut(&mut self, identifier: &str) -> Option<&mut RegistrationInvitation> {
        self.invitations
            .iter_mut()
            .find(|invitation| invitation.is_identified_by(identifier))
    }

    fn assert_active(&self) -> Result<(), TenantError> {
        if !self.active {
            Err(TenantError::NotActive)
        } else {
            Ok(())
        }
    }
}

impl PartialEq for Tenant {
    /// Two `Tenant`s are equal if they have the same tenant identifier.
    fn eq(&self, other: &Self) -> bool {
        return self.tenant_id == other.tenant_id
    }
}