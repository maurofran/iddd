use common::{constrained_string, validate};
use crate::domain::identity::{Group, GroupDescription, GroupMemberService, GroupName, TenantId, User};
use anyhow::Result;

constrained_string!(RoleName, 70);
constrained_string!(RoleDescription, 255);

const ROLE_GROUP_PREFIX: &str = "ROLE-INTERNAL-GROUP: ";

#[derive(Debug, Clone)]
pub struct Role {
    tenant_id: TenantId,
    name: RoleName,
    description: RoleDescription,
    supports_nesting: bool,
    group: Group,
}

impl Role {
    pub fn hydrate(tenant_id: TenantId, name: RoleName, description: RoleDescription, supports_nesting: bool, group: Group) -> Self {
        Self {
            tenant_id,
            name,
            description,
            supports_nesting,
            group,
        }
    }

    pub fn new(tenant_id: TenantId, name: RoleName, description: RoleDescription, supports_nesting: bool) -> Self {
        let mut group_name = ROLE_GROUP_PREFIX.to_owned();
        group_name.push_str(name.as_ref());
        let nested_group_name = GroupName::new(group_name.as_str()).unwrap();
        let mut group_description = "Role backing group for ".to_owned();
        group_description.push_str(name.as_ref());
        let nested_group_description = GroupDescription::new(group_description.as_str()).unwrap();
        let nested_group = Group::new(tenant_id.clone(), nested_group_name, nested_group_description).unwrap();
        Self {
            tenant_id,
            name,
            description,
            supports_nesting,
            group: nested_group,
        }
    }

    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    pub fn name(&self) -> &RoleName {
        &self.name
    }

    pub fn description(&self) -> &RoleDescription {
        &self.description
    }

    pub fn supports_nesting(&self) -> bool {
        self.supports_nesting
    }

    pub fn group(&self) -> &Group {
        &self.group
    }

    pub fn assign_group(&mut self, group: &Group, member_service: &GroupMemberService) -> Result<()> {
        validate::is_true(self.supports_nesting, "this role does not support group nesting")?;
        validate::equals("tenant_id", group.tenant_id(), self.tenant_id())?;
        self.group.add_group(group, member_service)
    }

    pub fn assign_user(&mut self, user: &User) -> Result<()> {
        validate::equals("tenant_id", user.tenant_id(), self.tenant_id())?;
        self.group.add_user(user)
    }

    pub fn is_in_role(&self, user: &User, member_service: &GroupMemberService) -> Result<bool> {
        self.group.is_member(user, member_service)
    }

    pub fn unassign_group(&mut self, group: &Group) -> Result<()> {
        validate::is_true(self.supports_nesting, "this role does not support group nesting")?;
        validate::equals("tenant_id", group.tenant_id(), self.tenant_id())?;
        self.group.remove_group(group)
    }

    pub fn unassign_user(&mut self, user: &User) -> Result<()> {
        validate::equals("tenant_id", user.tenant_id(), self.tenant_id())?;
        self.group.remove_user(user)
    }
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.tenant_id == other.tenant_id && self.name == other.name
    }
}