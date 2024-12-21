use crate::domain::identity::{GroupMember, GroupMemberService, GroupRepository, TenantId, User, UserRepository};
use anyhow::Result;
use common::{declare_simple_type, validate};
use std::ops::Deref;

const TENANT_ID: &str = "tenant_id";

declare_simple_type!(GroupName, 70);
declare_simple_type!(GroupDescription, 255);

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    tenant_id: TenantId,
    name: GroupName,
    description: Option<GroupDescription>,
    members: Vec<GroupMember>,
}

impl Group {
    pub fn hydrate(
        tenant_id: TenantId,
        name: GroupName,
        description: Option<GroupDescription>,
        members: Vec<GroupMember>,
    ) -> Self {
        Self {
            tenant_id,
            name,
            description,
            members,
        }
    }

    pub fn new(tenant_id: TenantId, name: GroupName, description: Option<GroupDescription>) -> Self {
        Self {
            tenant_id,
            name,
            description,
            members: Vec::new(),
        }
    }

    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    pub fn name(&self) -> &GroupName {
        &self.name
    }

    pub fn description(&self) -> &Option<GroupDescription> {
        &self.description
    }

    pub fn members(&self) -> &[GroupMember] {
        &self.members
    }

    pub async fn add_group<'a, G: GroupRepository, U: UserRepository>(&mut self, group: &Group, member_service: &GroupMemberService<'a, G, U>) -> Result<()> {
        validate::equals(TENANT_ID, group.tenant_id(), self.tenant_id())?;
        validate::is_false(
            member_service.is_member_group(group, &self.deref().into()).await?,
            "group recursion detected",
        )?;
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

    pub async fn is_member<'a, G: GroupRepository, U: UserRepository>(
        &self,
        user: &User,
        group_member_service: &GroupMemberService<'a, G, U>,
    ) -> Result<bool> {
        validate::equals(TENANT_ID, user.tenant_id(), self.tenant_id())?;
        validate::is_true(user.is_enabled(), "user is not enabled")?;

        let member: GroupMember = user.into();
        if self.members.contains(&member) {
            group_member_service.confirm_user(&self, user).await
        } else {
            group_member_service.is_user_in_nested_group(&self, user).await
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
