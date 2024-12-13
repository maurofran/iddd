use crate::domain::identity::{EmailAddress, PostalAddress, Telephone};

#[derive(Debug, PartialEq, Clone)]
pub struct ContactInformation {
    email_address: EmailAddress,
    postal_address: Option<PostalAddress>,
    primary_telephone: Option<Telephone>,
    secondary_telephone: Option<Telephone>,
}

impl ContactInformation {
    pub fn new(
        email_address: EmailAddress,
        postal_address: Option<PostalAddress>,
        primary_telephone: Option<Telephone>,
        secondary_telephone: Option<Telephone>,
    ) -> Self {
        Self {
            email_address,
            postal_address,
            primary_telephone,
            secondary_telephone,
        }
    }

    pub fn email_address(&self) -> &EmailAddress {
        &self.email_address
    }

    pub fn postal_address(&self) -> &Option<PostalAddress> {
        &self.postal_address
    }

    pub fn primary_telephone(&self) -> &Option<Telephone> {
        &self.primary_telephone
    }

    pub fn secondary_telephone(&self) -> &Option<Telephone> {
        &self.secondary_telephone
    }

    pub fn with_email_address(&self, email_address: EmailAddress) -> Self {
        Self {
            email_address,
            postal_address: self.postal_address.clone(),
            primary_telephone: self.primary_telephone.clone(),
            secondary_telephone: self.secondary_telephone.clone(),
        }
    }

    pub fn with_postal_address(&self, postal_address: Option<PostalAddress>) -> Self {
        Self {
            email_address: self.email_address.clone(),
            postal_address,
            primary_telephone: self.primary_telephone.clone(),
            secondary_telephone: self.secondary_telephone.clone(),
        }
    }

    pub fn with_primary_telephone(&self, primary_telephone: Option<Telephone>) -> Self {
        Self {
            email_address: self.email_address.clone(),
            postal_address: self.postal_address.clone(),
            primary_telephone,
            secondary_telephone: self.secondary_telephone.clone(),
        }
    }

    pub fn with_secondary_telephone(&self, secondary_telephone: Option<Telephone>) -> Self {
        Self {
            email_address: self.email_address.clone(),
            postal_address: self.postal_address.clone(),
            primary_telephone: self.primary_telephone.clone(),
            secondary_telephone,
        }
    }
}
