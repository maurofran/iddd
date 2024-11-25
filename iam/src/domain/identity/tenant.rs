use std::fmt::Display;
use thiserror::Error;
use crate::domain::identity::{InvitationDescription, InvitationDescriptor, InvitationRedefiner, InvitationValidityError, RegistrationInvitation, TenantId};

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
    #[error(transparent)]
    InvalidInvitationValidity(#[from] InvitationValidityError)
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
    pub fn redefine_invitation_as(&mut self, identifier: &str, redefiner_fn: InvitationRedefiner) -> Result<(), TenantError> {
        self.assert_active()?;
        if let Some(invitation) = self.invitation_mut(identifier) {
            invitation.redefine_as(redefiner_fn).map_err(|e| e.into())
        } else {
            Err(TenantError::InvitationExists(identifier.to_string()))
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

/// The name of a `Tenant`.
#[derive(Debug, Clone, PartialEq)]
pub struct TenantName(String);

/// The error for the name of a `Tenant`.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TenantNameError {
    #[error("tenant name is required")]
    Required,
    #[error("tenant name must be 100 characters or less")]
    TooLong,
}

impl TenantName {
    /// New creates a new `TenantName` from a given string.
    pub fn new(name: &str) -> Result<Self, TenantNameError> {
        if name.is_empty() {
            Err(TenantNameError::Required)
        } else if name.len() > 100 {
            Err(TenantNameError::TooLong)
        } else {
            Ok(Self(name.into()))
        }
    }

    /// Converts the `TenantName` into a string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for TenantName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for TenantName {
    type Error = TenantNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TenantName::new(value)
    }
}

impl AsRef<str> for TenantName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// The description of a `Tenant`.
#[derive(Debug, Clone, PartialEq)]
pub struct TenantDescription(String);

#[derive(Error, Debug, Clone, PartialEq)]
pub enum TenantDescriptionError {
    #[error("tenant description is required")]
    Required,
    #[error("tenant description must be 100 characters or less")]
    Long
}

impl TenantDescription {
    /// New creates a new `TenantDescription` from a given string.
    pub fn new(description: &str) -> Result<Self, TenantDescriptionError> {
        if description.is_empty() {
            Err(TenantDescriptionError::Required)
        } else if description.len() > 100 {
            Err(TenantDescriptionError::Long)
        } else {
            Ok(Self(description.into()))
        }
    }

    /// New creates a new `TenantDescription` from a given string, returning `None` if the string is
    /// empty.
    pub fn new_option(description: &str) -> Result<Option<Self>, TenantDescriptionError> {
        if description.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Self::new(description)?))
        }
    }

    /// Converts the `TenantDescription` into a string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for TenantDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for TenantDescription {
    type Error = TenantDescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TenantDescription::new(value)
    }
}

impl AsRef<str> for TenantDescription {
    fn as_ref(&self) -> &str {
        &self.0
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