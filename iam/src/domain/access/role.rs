use crate::domain::identity::{Group, GroupMemberService, TenantId, User};
use anyhow::Result;
use chrono::{DateTime, Utc};
use common::validate;

const TENANT_ID: &str = "tenant_id";
const NAME: &str = "name";
const DESCRIPTION: & str = "description";

const ROLE_GROUP_PREFIX: &str = "ROLE-INTERNAL-GROUP: ";

fn validate_name(name: &str) -> Result<String> {
    validate::not_empty(NAME, name)?;
    validate::max_length(NAME, name, 70)?;
    Ok(name.into())
}

fn validate_description(description: Option<&str>) -> Result<Option<String>> {
    match description {
        Some(description) => {
            validate::not_empty(DESCRIPTION, description)?;
            validate::max_length(DESCRIPTION, description, 255)?;
            Ok(Some(description.into()))
        }
        None => Ok(None),
    }
}

#[derive(Debug, Clone)]
pub struct Role {
    id: Option<i32>,
    version: i32,
    tenant_id: TenantId,
    name: String,
    description: Option<String>,
    supports_nesting: bool,
    group: Group,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Role {
    /// Re-create a [Role] instance with provided values from the persistent storage.
    pub fn hydrate(
        id: i32,
        version: i32,
        tenant_id: TenantId,
        name: &str,
        description: Option<&str>,
        supports_nesting: bool,
        group: Group,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self> {
        Ok(Self {
            id: Some(id),
            version,
            tenant_id,
            name: validate_name(name)?,
            description: validate_description(description)?,
            supports_nesting,
            group,
            created_at,
            updated_at,
        })
    }

    /// Creates a new [Role] instance with a provided nested group.
    pub fn new(
        tenant_id: TenantId,
        name: &str,
        description: Option<&str>,
        supports_nesting: bool,
    ) -> Result<Self> {
        let group_name = format!("{}{}", ROLE_GROUP_PREFIX, name);
        let group_description = format!("Role backing group for {}", name);
        let nested_group = Group::new(tenant_id.clone(), &group_name, &group_description)?;

        Ok(Self {
            id: None,
            version: 0,
            tenant_id,
            name: validate_name(name)?,
            description: validate_description(description)?,
            supports_nesting,
            group: nested_group,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Returns the tenant ID associated with this role.
    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    /// Returns the name of the [Role].
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description of the [Role].
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref().clone()
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
    pub fn assign_group(
        &mut self,
        group: &Group,
        member_service: &GroupMemberService,
    ) -> Result<()> {
        validate::is_true(
            self.supports_nesting,
            "this role does not support group nesting",
        )?;
        validate::equals(TENANT_ID, group.tenant_id(), self.tenant_id())?;
        self.group.add_group(group, member_service)
    }

    /// Assigns a [User] to the [Role].
    pub fn assign_user(&mut self, user: &User) -> Result<()> {
        validate::equals(TENANT_ID, user.tenant_id(), self.tenant_id())?;
        self.group.add_user(user)
    }

    /// Returns true if the [User] is a member of the [Role]'s group.
    pub fn is_in_role(&self, user: &User, member_service: &GroupMemberService) -> Result<bool> {
        self.group.is_member(user, member_service)
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
