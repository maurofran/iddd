use crate::domain::identity::{ContactInformation, Enablement, EncryptedPassword, FullName, Person, PlainPassword, TenantId};
use anyhow::Result;
use common::{constrained_string, validate};

constrained_string!(Username, 1, 255);

const NEW_PASSWORD: &str = "new_password";

/// User is the aggregate root entity representing a user in the system.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    tenant_id: TenantId,
    username: String,
    password: EncryptedPassword,
    enablement: Enablement,
    person: Person,
}

impl User {
    /// Hydrate a new user from the provided data.
    pub fn hydrate(tenant_id: TenantId, username: &str, password: EncryptedPassword,
                   enablement: Enablement, person: Person) -> Result<Self> {
        let mut user = Self {
            tenant_id,
            username: String::default(),
            password,
            enablement,
            person,
        };
        user.set_username(username)?;
        Ok(user)
    }

    /// Creates a new user with the given username, password, and person.
    pub fn new(tenant_id: TenantId, username: &str, password: PlainPassword,
               enablement: Enablement, person: Person) -> Result<Self> {
        let mut user = User {
            tenant_id,
            username: String::default(),
            password: EncryptedPassword::default(),
            enablement,
            person,
        };
        user.set_username(username)?;
        user.protect_password(PlainPassword::default(), password)?;
        Ok(user)
    }

    pub fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    fn set_username(&mut self, username: &str) -> Result<()> {
        const USERNAME: &str = "username";

        validate::not_empty(USERNAME, username)?;
        validate::max_length(USERNAME, username, 255)?;
        self.username = username.into();
        Ok(())
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

    pub fn change_password(&mut self, old_password: PlainPassword, new_password: PlainPassword) -> Result<()> {
        validate::not_empty(NEW_PASSWORD, new_password.as_ref())?;
        validate::is_true(self.password.verify(&old_password)?, "current password not confirmed")?;
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

    fn protect_password(&mut self, old_password: PlainPassword, new_password: PlainPassword) -> Result<()> {
        let old_ref = &old_password;
        let new_ref = &new_password;
        validate::not_equals(NEW_PASSWORD, new_ref, old_ref)?;
        validate::is_false(new_ref.is_weak(), "new password is weak")?;
        validate::not_equals(NEW_PASSWORD, new_ref.as_ref(), self.username.as_ref())?;
        self.password = new_ref.encrypt()?;
        Ok(())
    }
}
