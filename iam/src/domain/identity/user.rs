mod enablement;
mod password;
mod person;

use crate::domain::identity::{TenantId};
use anyhow::Result;
use chrono::{DateTime, Utc};
use common::{declare_simple_type, validate};

pub use enablement::Enablement;
pub use password::*;
pub use person::*;

declare_simple_type!(Username, 255);

const NEW_PASSWORD: &str = "new_password";

/// User is the aggregate root entity representing a user in the system.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: Option<i32>,
    version: i32,
    tenant_id: TenantId,
    username: Username,
    password: EncryptedPassword,
    enablement: Enablement,
    person: Person,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    /// Hydrate a new user from the provided data.
    pub fn hydrate(
        id: i32,
        version: i32,
        tenant_id: TenantId,
        username: Username,
        password: EncryptedPassword,
        enablement: Enablement,
        person: Person,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Some(id),
            version,
            tenant_id,
            username,
            password,
            enablement,
            person,
            created_at,
            updated_at,
        }
    }

    /// Creates a new user with the given username, password, and person.
    pub fn new(
        tenant_id: TenantId,
        username: Username,
        password: PlainPassword,
        enablement: Enablement,
        person: Person,
    ) -> Result<Self> {
        let mut user = Self {
            id: None,
            version: 0,
            tenant_id,
            username,
            password: EncryptedPassword::default(),
            enablement,
            person,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        user.protect_password(PlainPassword::default(), password)?;
        Ok(user)
    }

    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &EncryptedPassword {
        &self.password
    }

    pub fn enablement(&self) -> &Enablement {
        &self.enablement
    }

    pub fn person(&self) -> &Person {
        &self.person
    }

    pub fn change_password(
        &mut self,
        old_password: PlainPassword,
        new_password: PlainPassword,
    ) -> Result<()> {
        validate::not_empty(NEW_PASSWORD, new_password.as_ref())?;
        validate::is_true(
            self.password.verify(&old_password)?,
            "current password not confirmed",
        )?;
        self.protect_password(old_password, new_password)
    }

    pub fn change_personal_contact_information(&mut self, contact_information: ContactInformation) {
        self.person.change_contact_information(contact_information);
    }

    pub fn change_personal_name(&mut self, name: FullName) {
        self.person.change_name(name);
    }

    pub fn define_enablement(&mut self, enablement: Enablement) {
        self.enablement = enablement;
    }

    pub fn is_enabled(&self) -> bool {
        self.enablement.is_enabled()
    }

    fn protect_password(
        &mut self,
        old_password: PlainPassword,
        new_password: PlainPassword,
    ) -> Result<()> {
        let old_ref = &old_password;
        let new_ref = &new_password;
        validate::not_equals(NEW_PASSWORD, new_ref, old_ref)?;
        validate::is_false(new_ref.is_weak(), "new password is weak")?;
        validate::not_equals(NEW_PASSWORD, new_ref.as_ref(), self.username.as_ref())?;
        self.password = new_ref.encrypt()?;
        Ok(())
    }
}
