use std::fmt::Display;
use anyhow::Result;
use regex::Regex;
use common::validate;

/// Value object representing a postal address.
#[derive(Debug, Clone, PartialEq)]
pub struct PostalAddress {
    street_name: String,
    building_number: Option<String>,
    postal_code: String,
    city: String,
    state_province: String,
    country_code: String,
}

impl PostalAddress {
    pub fn new(
        street_name: &str,
        building_number: Option<&str>,
        postal_code: &str,
        city: &str,
        state_province: &str,
        country_code: &str,
    ) -> anyhow::Result<Self> {
        let mut address = Self::default();
        address.set_street_name(street_name)?;
        address.set_building_number(building_number)?;
        address.set_postal_code(postal_code)?;
        address.set_city(city)?;
        address.set_state_province(state_province)?;
        address.set_country_code(country_code)?;
        Ok(address)
    }

    pub fn street_name(&self) -> &str {
        &self.street_name
    }

    fn set_street_name(&mut self, street_name: &str) -> Result<()> {
        const STREET_NAME: &str = "street_name";
        validate::not_empty(STREET_NAME, street_name)?;
        validate::max_length(STREET_NAME, street_name, 150)?;
        self.street_name = street_name.into();
        Ok(())
    }

    pub fn building_number(&self) -> Option<&str> {
        self.building_number.as_deref().clone()
    }

    fn set_building_number(&mut self, building_number: Option<&str>) -> Result<()> {
        const BUILDING_NUMBER: &str = "building_number";
        match building_number {
            Some(building_number) => {
                validate::not_empty(BUILDING_NUMBER, building_number)?;
                validate::max_length(BUILDING_NUMBER, building_number, 18)?;
                self.building_number = Some(building_number.into());
            }
            None => self.building_number = None,
        }
        Ok(())
    }

    pub fn postal_code(&self) -> &str {
        &self.postal_code
    }

    fn set_postal_code(&mut self, postal_code: &str) -> Result<()> {
        const POSTAL_CODE: &str = "postal_code";
        validate::not_empty(POSTAL_CODE, postal_code)?;
        validate::max_length(POSTAL_CODE, postal_code, 10)?;
        self.postal_code = postal_code.into();
        Ok(())
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    fn set_city(&mut self, city: &str) -> Result<()> {
        const CITY: &str = "city";
        validate::not_empty(CITY, city)?;
        validate::max_length(CITY, city, 35)?;
        self.city = city.into();
        Ok(())
    }

    pub fn state_province(&self) -> &str {
        &self.state_province
    }

    fn set_state_province(&mut self, state_province: &str) -> Result<()> {
        const STATE_PROVINCE: &str = "state_province";
        validate::not_empty(STATE_PROVINCE, state_province)?;
        validate::max_length(STATE_PROVINCE, state_province, 18)?;
        self.state_province = state_province.into();
        Ok(())
    }

    pub fn country_code(&self) -> &str {
        &self.country_code
    }

    fn set_country_code(&mut self, country_code: &str) -> Result<()> {
        const COUNTRY_CODE: &str = "country_code";
        let pattern = Regex::new(r"^[A-Z]{2}$")?;

        validate::not_empty(COUNTRY_CODE, country_code)?;
        validate::length_between(COUNTRY_CODE, country_code, 2, 2)?;
        validate::matches(COUNTRY_CODE, country_code, pattern)?;
        self.country_code = country_code.into();
        Ok(())
    }
}

impl Default for PostalAddress {
    fn default() -> Self {
        Self {
            street_name: String::default(),
            building_number: None,
            postal_code: String::default(),
            city: String::default(),
            state_province: String::default(),
            country_code: String::default(),
        }
    }
}

impl Display for PostalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} - {} {} ({}) - {}",
            self.street_name,
            self.building_number.as_deref().map_or("".to_owned(), |b| format!(", {}", b)),
            self.postal_code,
            self.city,
            self.state_province,
            self.country_code,
        )
    }
}