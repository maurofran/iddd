use std::fmt::Display;

declare_simple_type!(StreetName, 150);
declare_simple_type!(BuildingNumber, 18);
declare_simple_type!(PostalCode, 10);
declare_simple_type!(City, 35);
declare_simple_type!(StateProvince, 18);
declare_simple_type!(CountryCode, 2, r"^[A-Z]{2}$");
use common::declare_simple_type;

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

impl Display for PostalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} - {} {} ({}) - {}",
            self.street_name,
            self.building_number
                .as_deref()
                .map_or("".to_owned(), |b| format!(", {}", b)),
            self.postal_code,
            self.city,
            self.state_province,
            self.country_code,
        )
    }
}
