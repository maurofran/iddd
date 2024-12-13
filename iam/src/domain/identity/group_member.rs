use crate::domain::identity::{Group, User};

#[derive(Debug, Clone, PartialEq)]
pub enum GroupMember {
    User(String),
    Group(String),
}

impl GroupMember {
    pub fn is_group_with_name(&self, name: &str) -> bool {
        match self {
            GroupMember::Group(group_name) => group_name == name,
            _ => false,
        }
    }
}

impl From<&User> for GroupMember {
    fn from(user: &User) -> Self {
        GroupMember::User(user.username().into())
    }
}

impl From<&Group> for GroupMember {
    fn from(group: &Group) -> Self {
        GroupMember::Group(group.name().into())
    }
}
