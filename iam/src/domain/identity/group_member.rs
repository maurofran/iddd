use crate::domain::identity::{Group, GroupName, User, Username};

#[derive(Debug, Clone, PartialEq)]
pub enum GroupMember {
    User(Username),
    Group(GroupName),
}

impl GroupMember {
    pub fn is_group_with_name(&self, name: &GroupName) -> bool {
        match self {
            GroupMember::Group(group_name) => group_name == name,
            _ => false,
        }
    }
}

impl From<&User> for GroupMember {
    fn from(user: &User) -> Self {
        GroupMember::User(user.username().clone())
    }
}

impl From<&Group> for GroupMember {
    fn from(group: &Group) -> Self {
        GroupMember::Group(group.name().clone())
    }
}