use crate::domain::identity::{ContactInformation, EmailAddress, FullName};

#[derive(Debug, PartialEq, Clone)]
pub struct Person {
    name: FullName,
    contact_information: ContactInformation,
}

impl Person {
    /// Creates a new `Person` with the given name and contact information.
    pub fn new(name: FullName, contact_information: ContactInformation) -> Self {
        Person {
            name,
            contact_information,
        }
    }

    /// Returns the person's full name.
    pub fn name(&self) -> &FullName {
        &self.name
    }

    /// Returns the person's email address.
    pub fn email_address(&self) -> &EmailAddress {
        self.contact_information.email_address()
    }

    /// Returns the person's postal address.
    pub fn contact_information(&self) -> &ContactInformation {
        &self.contact_information
    }

    /// Changes the person's full name.
    pub(in crate::domain::identity) fn change_name(&mut self, new_full_name: FullName) {
        self.name = new_full_name;
    }

    /// Changes the person's contact information.
    pub(in crate::domain::identity) fn change_contact_information(
        &mut self, new_contact_information: ContactInformation) {

        self.contact_information = new_contact_information;
    }
}