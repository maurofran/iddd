use crate::domain::identity::{Group, GroupDescription, GroupMemberService, GroupName, GroupRepository, TenantId, User, UserRepository};
use anyhow::Result;
use common::{declare_simple_type, validate};

const TENANT_ID: &str = "tenant_id";

const ROLE_GROUP_PREFIX: &str = "ROLE-INTERNAL-GROUP: ";

declare_simple_type!(RoleName, 70);
declare_simple_type!(RoleDescription, 255);

#[derive(Debug, Clone)]
pub struct Role {
    tenant_id: TenantId,
    name: RoleName,
    description: Option<RoleDescription>,
    supports_nesting: bool,
    group: Group,
}

impl Role {
    /// Re-create a [Role] instance with provided values from the persistent storage.
    pub fn hydrate(
        tenant_id: TenantId,
        name: RoleName,
        description: Option<RoleDescription>,
        supports_nesting: bool,
        group: Group,
    ) -> Self {
        Self {
            tenant_id,
            name,
            description,
            supports_nesting,
            group,
        }
    }

    /// Creates a new [Role] instance with a provided nested group.
    pub fn new(
        tenant_id: TenantId,
        name: RoleName,
        description: Option<RoleDescription>,
        supports_nesting: bool,
    ) -> Result<Self> {
        let group_name = &format!("{}{}", ROLE_GROUP_PREFIX, name);
        let group_description = &format!("Role backing group for {}", name);
        let nested_group = Group::new(tenant_id.clone(), GroupName::new(group_name)?,
                                      Some(GroupDescription::new(group_description)?));

        Ok(Self {
            tenant_id,
            name,
            description,
            supports_nesting,
            group: nested_group,
        })
    }

    /// Returns the tenant ID associated with this role.
    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    /// Returns the name of the [Role].
    pub fn name(&self) -> &RoleName {
        &self.name
    }

    /// Returns the description of the [Role].
    pub fn description(&self) -> &Option<RoleDescription> {
        &self.description
    }

    /// Returns true if the [Role] supports group nesting.
    pub fn supports_nesting(&self) -> bool {
        self.supports_nesting
    }

    /// Returns the group associated with this [Role].
    pub fn group(&self) -> &Group {
        &self.group
    }

    /// Assigns a [Group] to the [Role].
    pub async fn assign_group<'a, G: GroupRepository, U: UserRepository>(
        &mut self,
        group: &Group,
        member_service: &GroupMemberService<'a, G, U>,
    ) -> Result<()> {
        validate::is_true(
            self.supports_nesting,
            "this role does not support group nesting",
        )?;
        validate::equals(TENANT_ID, group.tenant_id(), self.tenant_id())?;
        self.group.add_group(group, member_service).await
    }

    /// Assigns a [User] to the [Role].
    pub fn assign_user(&mut self, user: &User) -> Result<()> {
        validate::equals(TENANT_ID, user.tenant_id(), self.tenant_id())?;
        self.group.add_user(user)
    }

    /// Returns true if the [User] is a member of the [Role]'s group.
    pub async fn is_in_role<'a, G: GroupRepository, U: UserRepository>(&self, user: &User, member_service: &GroupMemberService<'a, G, U>) -> Result<bool> {
        self.group.is_member(user, member_service).await
    }

    /// Removes a [Group] from the [Role].
    pub fn unassign_group(&mut self, group: &Group) -> Result<()> {
        validate::is_true(
            self.supports_nesting,
            "this role does not support group nesting",
        )?;
        validate::equals(TENANT_ID, group.tenant_id(), self.tenant_id())?;
        self.group.remove_group(group)
    }

    /// Removes a [User] from the [Role].
    pub fn unassign_user(&mut self, user: &User) -> Result<()> {
        validate::equals(TENANT_ID, user.tenant_id(), self.tenant_id())?;
        self.group.remove_user(user)
    }
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.tenant_id == other.tenant_id && self.name == other.name
    }
}
