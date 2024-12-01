use std::ops::Deref;
use common::{constrained_string, validate};
use crate::domain::identity::{GroupMember, GroupMemberService, TenantId, User};
use anyhow::Result;

constrained_string!(GroupName, 70);
constrained_string!(GroupDescription, 255);

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    tenant_id: TenantId,
    name: GroupName,
    description: GroupDescription,
    members: Vec<GroupMember>,
}

impl Group {
    pub fn hydrate(tenant_id: TenantId, name: GroupName, description: GroupDescription, members: Vec<GroupMember>) -> Self {
        Self {
            tenant_id,
            name,
            description,
            members,
        }
    }

    pub fn new(tenant_id: TenantId, name: GroupName, description: GroupDescription) -> Result<Self> {
        Ok(Self {
            tenant_id,
            name,
            description,
            members: Vec::new(),
        })
    }

    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    pub fn name(&self) -> &GroupName {
        &self.name
    }

    pub fn description(&self) -> &GroupDescription {
        &self.description
    }

    pub fn members(&self) -> &Vec<GroupMember> {
        &self.members
    }

    pub fn add_group(&mut self, group: &Group, member_service: &GroupMemberService) -> Result<()> {
        validate::equals("tenant_id", group.tenant_id(), self.tenant_id())?;
        validate::is_false(member_service.is_member_group(group, &self.deref().into())?, "group recursion detected")?;
        let member: GroupMember = group.into();
        if !self.members.contains(&member) {
            self.members.push(member);
        }
        Ok(())
    }

    pub fn add_user(&mut self, user: &User) -> Result<()> {
        validate::equals("tenant_id", user.tenant_id(), self.tenant_id())?;
        validate::is_true(user.is_enabled(), "user is not enabled")?;
        let member: GroupMember = user.into();
        if!self.members.contains(&member) {
            self.members.push(member);
        }
        Ok(())
    }

    pub fn is_member(&self, user: &User, group_member_service: &GroupMemberService) -> Result<bool> {
        validate::equals("tenant_id", user.tenant_id(), self.tenant_id())?;
        validate::is_true(user.is_enabled(), "user is not enabled")?;

        let member: GroupMember = user.into();
        if self.members.contains(&member) {
            group_member_service.confirm_user(&self, user)
        } else {
            group_member_service.is_user_in_nested_group(&self, user)
        }
    }

    pub fn remove_group(&mut self, group: &Group) -> Result<()> {
        validate::equals("tenant_id", group.tenant_id(), self.tenant_id())?;

        let member: GroupMember = group.into();
        self.members.retain(|m| m != &member);
        Ok(())
    }

    pub fn remove_user(&mut self, user: &User) -> Result<()> {
        validate::equals("tenant_id", user.tenant_id(), self.tenant_id())?;

        let member: GroupMember = user.into();
        self.members.retain(|m| m!= &member);
        Ok(())
    }
}