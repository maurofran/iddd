use crate::domain::identity::{Group, GroupName, User, Username};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupMember {
    User(Username),
    Group(GroupName),
}

impl GroupMember {
    pub fn is_group_with_name(&self, name: &str) -> bool {
        match self {
            GroupMember::Group(group_name) => group_name.as_ref() == name,
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
