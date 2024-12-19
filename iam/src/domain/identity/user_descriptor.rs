use crate::domain::identity::{EmailAddress, TenantId, User, Username};

pub struct UserDescriptor {
    pub tenant_id: TenantId,
    pub username: Username,
    pub email_address: EmailAddress,
}

impl From<User> for UserDescriptor {
    fn from(user: User) -> Self {
        UserDescriptor {
            tenant_id: user.tenant_id().clone(),
            username: user.username().clone(),
            email_address: user.person().email_address().clone(),
        }
    }
}
