use anyhow::Result;
use common::validate;
use regex::Regex;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct ContactInformation {
    email_address: EmailAddress,
    postal_address: Option<PostalAddress>,
    primary_telephone: Option<Telephone>,
    secondary_telephone: Option<Telephone>,
}

impl ContactInformation {
    pub fn new(email_address: EmailAddress, postal_address: Option<PostalAddress>,
               primary_telephone: Option<Telephone>, secondary_telephone: Option<Telephone>) -> Self {
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
    pub fn new(street_name: StreetName, building_number: Option<BuildingNumber>,
               postal_code: PostalCode, city: City, state_province: StateProvince,
               country_code: CountryCode) -> Self {
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

const STREET_NAME: &str = "street_name";

/// Custom value object representing a street name.
#[derive(Debug, Clone, PartialEq)]
pub struct StreetName(String);

impl StreetName {
    pub fn new(street_name: &str) -> Result<Self> {
        validate::not_empty(STREET_NAME, street_name)?;
        validate::max_length(STREET_NAME, street_name, 150)?;
        Ok(StreetName(street_name.into()))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for StreetName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for StreetName {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        StreetName::new(value)
    }
}

impl AsRef<str> for StreetName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

const BUILDING_NUMBER: &str = "building_number";

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingNumber(String);

impl BuildingNumber {
    pub fn new(building_number: &str) -> Result<Self, anyhow::Error> {
        validate::not_empty(BUILDING_NUMBER, building_number)?;
        validate::max_length(BUILDING_NUMBER, building_number, 18)?;
        Ok(BuildingNumber(building_number.into()))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for BuildingNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for BuildingNumber {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        BuildingNumber::new(value)
    }
}

impl AsRef<str> for BuildingNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

const POSTAL_CODE: &str = "postal_code";

#[derive(Debug, Clone, PartialEq)]
pub struct PostalCode(String);

impl PostalCode {
    pub fn new(postal_code: &str) -> Result<Self> {
        validate::not_empty(POSTAL_CODE, postal_code)?;
        validate::max_length(POSTAL_CODE, postal_code, 10)?;
        Ok(PostalCode(postal_code.into()))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for PostalCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for PostalCode {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        PostalCode::new(value)
    }
}

impl AsRef<str> for PostalCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

const CITY: &str = "city";

#[derive(Debug, Clone, PartialEq)]
pub struct City(String);

impl City {
    pub fn new(city: &str) -> Result<Self> {
        validate::not_empty(CITY, city)?;
        validate::max_length(CITY, city, 70)?;
        Ok(City(city.into()))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for City {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        City::new(value)
    }
}

impl AsRef<str> for City {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

const STATE_PROVINCE: &str = "state_province";

#[derive(Debug, Clone, PartialEq)]
pub struct StateProvince(String);

impl StateProvince {
    pub fn new(state_province: &str) -> Result<Self> {
        validate::not_empty(STATE_PROVINCE, state_province)?;
        validate::max_length(STATE_PROVINCE, state_province, 18)?;
        Ok(StateProvince(state_province.into()))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for StateProvince {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for StateProvince {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        StateProvince::new(value)
    }
}

impl AsRef<str> for StateProvince {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

const COUNTRY_CODE: &str = "country_code";

fn country_code_regex() -> Regex {
    Regex::new(r"^[A-Z]{2}$").unwrap()
}

#[derive(Debug, Clone, PartialEq)]
pub struct CountryCode(String);

impl CountryCode {
    pub fn new(country_code: &str) -> Result<Self> {
        validate::not_empty(COUNTRY_CODE, country_code)?;
        validate::length_between(COUNTRY_CODE, country_code, 2, 2)?;
        validate::matches(COUNTRY_CODE, country_code, country_code_regex())?;
        Ok(CountryCode(country_code.into()))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for CountryCode {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CountryCode::new(value)
    }
}

impl AsRef<str> for CountryCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

const EMAIL: &str = "email";

fn email_regex() -> Regex {
    Regex::new(r"\w+([-+.']\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*").unwrap()
}

/// Custom type for email addresses.
#[derive(Debug, Clone, PartialEq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email: &str) -> Result<Self> {
        validate::not_empty(EMAIL, email)?;
        validate::max_length(EMAIL, email, 255)?;
        validate::matches(EMAIL, email, email_regex())?;
        Ok(EmailAddress(email.into()))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

const TELEPHONE: &str = "telephone";

fn telephone_regex() -> Regex {
    Regex::new(r"((\(\d{3}\))|(\d{3}-))\d{3}-\d{4}").unwrap()
}

/// Custom type for phone numbers.
#[derive(Debug, Clone, PartialEq)]
pub struct Telephone(String);

impl Telephone {
    /// Creates a new `Telephone` from a string slice.
    pub fn new(telephone: &str) -> Result<Self> {
        validate::not_empty(TELEPHONE, telephone)?;
        validate::length_between(TELEPHONE, telephone, 5, 20)?;
        validate::matches(TELEPHONE, telephone, telephone_regex())?;
        Ok(Telephone(telephone.into()))
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Display for Telephone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Telephone {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl AsRef<str> for Telephone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}