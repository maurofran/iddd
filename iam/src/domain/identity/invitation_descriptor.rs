use crate::domain::identity::{InvitationDescription, InvitationId, RegistrationInvitation, TenantId, Validity};

/// Value object representing an invitation.
#[derive(Debug, Clone, PartialEq)]
pub struct InvitationDescriptor {
    pub tenant_id: TenantId,
    pub invitation_id: InvitationId,
    pub invitation_description: InvitationDescription,
    pub invitation_validity: Validity,
}

impl InvitationDescriptor {
    pub fn new(tenant_id: &TenantId, invitation: &RegistrationInvitation) -> Self {
        Self {
            tenant_id: tenant_id.clone(),
            invitation_id: invitation.invitation_id().clone(),
            invitation_description: invitation.description().clone(),
            invitation_validity: invitation.validity().clone(),
        }
    }
}
