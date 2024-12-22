use crate::domain::identity::{
    BuildingNumber, City, ContactInformation, CountryCode, EmailAddress, Enablement,
    EncryptedPassword, FirstName, FullName, LastName, Person, PostalAddress, PostalCode,
    StateProvince, StreetName, Telephone, TenantId, TenantRepositoryError, User,
    UserRepositoryError, Username, Validity,
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{query_file, query_file_as, Pool};
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

/// Implementation of [UserRepository] for PostgresSQL
pub struct UserRepository<'a> {
    pool: &'a Pool<sqlx::Postgres>,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a Pool<sqlx::Postgres>) -> Self {
        UserRepository { pool }
    }

    fn check_for_affected_rows(result: PgQueryResult, user: &User) -> Result<()> {
        if result.rows_affected() == 0 {
            return Err(UserRepositoryError::NotFound(
                user.tenant_id().clone(),
                user.username().clone(),
            )
                .into());
        }
        Ok(())
    }

}

impl<'a> crate::domain::identity::UserRepository for UserRepository<'a> {
    async fn add(&self, user: &User) -> Result<()> {
        let tenant_id: Uuid = user.tenant_id().into();
        let enablement = user.enablement();
        let person = user.person();
        let name = person.name();
        let contact_information = person.contact_information();
        let postal_address = contact_information.postal_address().as_ref();
        query_file!(
            "sql/postgres/insert_user.sql",
            tenant_id,
            user.username().as_ref(),
            user.password().as_ref(),
            enablement.enabled(),
            enablement.validity().and_then(|v| v.start_date()),
            enablement.validity().and_then(|v| v.end_date()),
            name.first_name().as_ref(),
            name.last_name().as_ref(),
            contact_information.email_address().as_ref(),
            postal_address.map(|pa| pa.street_name().as_ref()),
            postal_address
                .and_then(|pa| pa.building_number().as_ref())
                .map(|bn| bn.as_ref()),
            postal_address.map(|pa| pa.postal_code().as_ref()),
            postal_address.map(|pa| pa.city().as_ref()),
            postal_address.map(|pa| pa.state_province().as_ref()),
            postal_address.map(|pa| pa.country_code().as_ref()),
            contact_information
                .primary_telephone()
                .as_ref()
                .map(|t| t.as_ref()),
            contact_information
                .secondary_telephone()
                .as_ref()
                .map(|t| t.as_ref()),
        )
        .execute(self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, user: &User) -> Result<()> {
        let tenant_id: Uuid = user.tenant_id().into();
        let enablement = user.enablement();
        let person = user.person();
        let name = person.name();
        let contact_information = person.contact_information();
        let postal_address = contact_information.postal_address().as_ref();
        let result = query_file!(
            "sql/postgres/update_user.sql",
            tenant_id,
            user.username().as_ref(),
            user.password().as_ref(),
            enablement.enabled(),
            enablement.validity().and_then(|v| v.start_date()),
            enablement.validity().and_then(|v| v.end_date()),
            name.first_name().as_ref(),
            name.last_name().as_ref(),
            contact_information.email_address().as_ref(),
            postal_address.map(|pa| pa.street_name().as_ref()),
            postal_address
                .and_then(|pa| pa.building_number().as_ref())
                .map(|bn| bn.as_ref()),
            postal_address.map(|pa| pa.postal_code().as_ref()),
            postal_address.map(|pa| pa.city().as_ref()),
            postal_address.map(|pa| pa.state_province().as_ref()),
            postal_address.map(|pa| pa.country_code().as_ref()),
            contact_information
                .primary_telephone()
                .as_ref()
                .map(|t| t.as_ref()),
            contact_information
                .secondary_telephone()
                .as_ref()
                .map(|t| t.as_ref()),
        )
        .execute(self.pool)
        .await?;
        Self::check_for_affected_rows(result, user)
    }

    async fn remove(&self, user: &User) -> Result<()> {
        let tenant_id: Uuid = user.tenant_id().into();
        let result = query_file!(
            "sql/postgres/delete_user.sql",
            tenant_id,
            user.username().as_ref(),
        )
        .execute(self.pool)
        .await?;
        Self::check_for_affected_rows(result, user)
    }

    async fn find_by_username(&self, tenant_id: &TenantId, username: &Username) -> Result<User> {
        let tenant_id_uuid: Uuid = tenant_id.into();
        let result = query_file_as!(
            UserAndPersonRow,
            "sql/postgres/find_user_by_username.sql",
            tenant_id_uuid,
            username.as_ref(),
        )
        .fetch_one(self.pool)
        .await;
        result
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => {
                    UserRepositoryError::NotFound(tenant_id.clone(), username.clone()).into()
                }
                _ => err.into(),
            })
            .and_then(|row| row.try_into())
    }

    async fn find_all_similarly_named(
        &self,
        tenant_id: &TenantId,
        first_name_prefix: &str,
        last_name_prefix: &str,
    ) -> Result<Vec<User>> {
        let tenant_id_uuid: Uuid = tenant_id.into();
        let rows = query_file_as!(
            UserAndPersonRow,
            "sql/postgres/find_all_users.sql",
            tenant_id_uuid,
            first_name_prefix.to_owned() + "%",
            last_name_prefix.to_owned() + "%"
        )
        .fetch_all(self.pool)
        .await?;
        let mut result: Vec<User> = Vec::new();
        for row in rows {
            result.push(row.try_into()?)
        }
        Ok(result)
    }
}

#[derive(sqlx::FromRow)]
struct UserAndPersonRow {
    tenant_id: Uuid,
    username: String,
    password: String,
    enabled: bool,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    first_name: String,
    last_name: String,
    email_address: String,
    street_name: Option<String>,
    building_number: Option<String>,
    postal_code: Option<String>,
    city: Option<String>,
    state_province: Option<String>,
    country_code: Option<String>,
    primary_telephone: Option<String>,
    secondary_telephone: Option<String>,
}

impl TryInto<User> for UserAndPersonRow {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<User> {
        let tenant_id: TenantId = self.tenant_id.into();
        let username = Username::new(&self.username)?;
        let password = EncryptedPassword::new(&self.password);
        let validity = Validity::new(self.start_date, self.end_date)?;
        let enablement = Enablement::new(self.enabled, validity);
        let person: Person = self.try_into()?;
        Ok(User::hydrate(
            tenant_id, username, password, enablement, person,
        ))
    }
}

impl TryInto<Person> for UserAndPersonRow {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Person> {
        let first_name = FirstName::new(&self.first_name)?;
        let last_name = LastName::new(&self.last_name)?;
        let full_name = FullName::new(first_name, last_name);

        let email_address = EmailAddress::new(&self.email_address)?;
        let primary_telephone = match self.primary_telephone {
            Some(telephone) => Some(Telephone::new(&telephone)?),
            None => None,
        };
        let secondary_telephone = match self.secondary_telephone {
            Some(telephone) => Some(Telephone::new(&telephone)?),
            None => None,
        };
        let postal_address = match (
            self.street_name,
            self.postal_code,
            self.city,
            self.state_province,
            self.country_code,
        ) {
            (
                Some(street_name),
                Some(postal_code),
                Some(city),
                Some(state_province),
                Some(country_code),
            ) => {
                let building_number = match self.building_number {
                    Some(value) => Some(BuildingNumber::new(&value)?),
                    None => None,
                };
                Some(PostalAddress::new(
                    StreetName::new(&street_name)?,
                    building_number,
                    PostalCode::new(&postal_code)?,
                    City::new(&city)?,
                    StateProvince::new(&state_province)?,
                    CountryCode::new(&country_code)?,
                ))
            }
            _ => None,
        };
        let contact_information = ContactInformation::new(
            email_address,
            postal_address,
            primary_telephone,
            secondary_telephone,
        );

        Ok(Person::new(full_name, contact_information))
    }
}