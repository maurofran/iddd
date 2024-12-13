use crate::domain::identity::{GroupMember, GroupMemberService, TenantId, User};
use anyhow::Result;
use common::validate;
use std::ops::Deref;

const TENANT_ID: &str = "tenant_id";

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    tenant_id: TenantId,
    name: String,
    description: String,
    members: Vec<GroupMember>,
}

impl Group {
    pub fn hydrate(tenant_id: TenantId, name: &str, description: &str, members: Vec<GroupMember>) -> Result<Self> {
        let mut group = Self {
            tenant_id,
            name: String::default(),
            description: String::default(),
            members,
        };
        group.set_name(name)?;
        group.set_description(description)?;
        Ok(group)
    }

    pub fn new(tenant_id: TenantId, name: &str, description: &str) -> Result<Self> {
        let mut group = Self {
            tenant_id,
            name: String::default(),
            description: String::default(),
            members: Vec::new(),
        };
        group.set_name(name)?;
        group.set_description(description)?;
        Ok(group)
    }

    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) -> Result<()> {
        const NAME: &str = "name";
        validate::not_empty(NAME, name)?;
        validate::max_length(NAME, name, 70)?;
        self.name = name.into();
        Ok(())
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    fn set_description(&mut self, description: &str) -> Result<()> {
        const DESCRIPTION: &str = "description";
        validate::not_empty(DESCRIPTION, description)?;
        validate::max_length(DESCRIPTION, description, 255)?;
        self.description = description.into();
        Ok(())
    }

    pub fn members(&self) -> &[GroupMember] {
        &self.members
    }

    pub fn add_group(&mut self, group: &Group, member_service: &GroupMemberService) -> Result<()> {
        validate::equals(TENANT_ID, group.tenant_id(), self.tenant_id())?;
        validate::is_false(member_service.is_member_group(group, &self.deref().into())?, "group recursion detected")?;
        let member: GroupMember = group.into();
        if !self.members.contains(&member) {
            self.members.push(member);
        }
        Ok(())
    }

    pub fn add_user(&mut self, user: &User) -> Result<()> {
        validate::equals(TENANT_ID, user.tenant_id(), self.tenant_id())?;
        validate::is_true(user.is_enabled(), "user is not enabled")?;
        let member: GroupMember = user.into();
        if !self.members.contains(&member) {
            self.members.push(member);
        }
        Ok(())
    }

    pub fn is_member(&self, user: &User, group_member_service: &GroupMemberService) -> Result<bool> {
        validate::equals(TENANT_ID, user.tenant_id(), self.tenant_id())?;
        validate::is_true(user.is_enabled(), "user is not enabled")?;

        let member: GroupMember = user.into();
        if self.members.contains(&member) {
            group_member_service.confirm_user(&self, user)
        } else {
            group_member_service.is_user_in_nested_group(&self, user)
        }
    }

    pub fn remove_group(&mut self, group: &Group) -> Result<()> {
        validate::equals(TENANT_ID, group.tenant_id(), self.tenant_id())?;

        let member: GroupMember = group.into();
        self.members.retain(|m| m != &member);
        Ok(())
    }

    pub fn remove_user(&mut self, user: &User) -> Result<()> {
        validate::equals(TENANT_ID, user.tenant_id(), self.tenant_id())?;

        let member: GroupMember = user.into();
        self.members.retain(|m| m != &member);
        Ok(())
    }
}