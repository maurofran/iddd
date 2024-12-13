use crate::domain::identity::{RegistrationInvitation, TenantId, Validity};

/// Value object representing an invitation.
#[derive(Debug, Clone, PartialEq)]
pub struct InvitationDescriptor {
    pub tenant_id: TenantId,
    pub invitation_id: String,
    pub invitation_description: String,
    pub invitation_validity: Validity,
}

impl InvitationDescriptor {
    pub fn new(tenant_id: &TenantId, invitation: &RegistrationInvitation) -> Self {
        Self {
            tenant_id: tenant_id.clone(),
            invitation_id: invitation.invitation_id().to_string(),
            invitation_description: invitation.description().to_string(),
            invitation_validity: invitation.validity().clone(),
        }
    }
}
