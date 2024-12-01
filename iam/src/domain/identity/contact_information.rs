use common::constrained_string;
use std::fmt::Display;

constrained_string!(StreetName, 150);
constrained_string!(BuildingNumber, 18);
constrained_string!(PostalCode, 10);
constrained_string!(City, 35);
constrained_string!(StateProvince, 18);
constrained_string!(CountryCode, 2, 2, r"^[A-Z]{2}$");
constrained_string!(EmailAddress, 1, 255, r"\w+([-+.']\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*");
constrained_string!(Telephone, 5, 20, r"((\(\d{3}\))|(\d{3}-))\d{3}-\d{4}");

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

/// Value object representing a postal address.
#[derive(Debug, Clone, PartialEq)]
pub struct PostalAddress {
    street_name: StreetName,
    building_number: Option<BuildingNumber>,
    postal_code: PostalCode,
    city: City,
    state_province: StateProvince,
    country_code: CountryCode,
}

impl PostalAddress {
    pub fn new(
        street_name: StreetName,
        building_number: Option<BuildingNumber>,
        postal_code: PostalCode,
        city: City,
        state_province: StateProvince,
        country_code: CountryCode,
    ) -> Self {
        Self {
            street_name,
            building_number,
            postal_code,
            city,
            state_province,
            country_code,
        }
    }

    pub fn street_name(&self) -> &StreetName {
        &self.street_name
    }

    pub fn building_number(&self) -> &Option<BuildingNumber> {
        &self.building_number
    }

    pub fn postal_code(&self) -> &PostalCode {
        &self.postal_code
    }

    pub fn city(&self) -> &City {
        &self.city
    }

    pub fn state_province(&self) -> &StateProvince {
        &self.state_province
    }

    pub fn country_code(&self) -> &CountryCode {
        &self.country_code
    }
}
