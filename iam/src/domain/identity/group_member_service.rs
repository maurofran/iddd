use crate::domain::identity::{
    Group, GroupMember, GroupRepository, GroupRepositoryError, User, UserRepository,
    UserRepositoryError,
};
use anyhow::Result;

pub struct GroupMemberService<'a, G: GroupRepository, U: UserRepository> {
    group_repository: &'a G,
    user_repository: &'a U,
}

impl<'a, G: GroupRepository, U: UserRepository> GroupMemberService<'a, G, U> {
    pub fn new(group_repository: &'a G, user_repository: &'a U) -> Self {
        GroupMemberService {
            group_repository,
            user_repository,
        }
    }

    pub async fn confirm_user(&self, group: &Group, user: &User) -> Result<bool> {
        self.user_repository
            .find_by_username(group.tenant_id(), user.username())
            .await
            .map(|user| user.is_enabled())
            .or_else(|error| match error.downcast_ref::<UserRepositoryError>() {
                Some(UserRepositoryError::NotFound(_, _)) => Ok(false),
                _ => Err(error),
            })
    }

    pub async fn is_member_group(&self, group: &Group, member: &GroupMember) -> Result<bool> {
        for group_member in group.members() {
            if group_member == member {
                return Ok(true);
            }
            match group_member {
                GroupMember::Group(name) => {
                    let member_group = self.group_repository.find_by_name(group.tenant_id(), name);
                    match member_group {
                        Ok(nested_group) => {
                            let is_member = Box::pin(self.is_member_group(&nested_group, member)).await?;
                            if is_member {
                                return Ok(true);
                            }
                        }
                        Err(error) => match error.downcast_ref::<GroupRepositoryError>() {
                            Some(GroupRepositoryError::NotFound(_, _)) => {}
                            _ => return Err(error),
                        },
                    }
                }
                _ => {}
            }
        }
        Ok(false)
    }

    pub async fn is_user_in_nested_group(&self, group: &Group, user: &User) -> Result<bool> {
        for member in group.members() {
            match member {
                GroupMember::Group(name) => {
                    let member_group = self.group_repository.find_by_name(group.tenant_id(), name);
                    match member_group {
                        Ok(nested_group) => {
                            let is_member = nested_group.is_member(user, &self).await?;
                            if is_member {
                                return Ok(true);
                            }
                        }
                        Err(error) => match error.downcast_ref::<GroupRepositoryError>() {
                            Some(GroupRepositoryError::NotFound(_, _)) => {}
                            _ => return Err(error),
                        },
                    }
                }
                _ => {}
            }
        }
        Ok(false)
    }
}
