use crate::domain::identity::{EmailAddress, TenantId, Username};

pub struct UserDescriptor {
    pub tenant_id: TenantId,
    pub username: Username,
    pub email_address: EmailAddress,
}