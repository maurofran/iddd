use crate::domain::identity::{
    InvitationDescription, InvitationId, RegistrationInvitation, Tenant, TenantDescription,
    TenantId, TenantName, Validity,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use sqlx::{query_file, query_file_as, Pool};
use uuid::Uuid;

/// Implementation of [TenantRepository] for PostgresSQL
pub struct TenantRepository<'a> {
    pool: &'a Pool<sqlx::Postgres>,
}

impl<'a> TenantRepository<'a> {
    pub fn new(pool: &'a Pool<sqlx::Postgres>) -> Self {
        TenantRepository { pool }
    }
}

impl<'a> crate::domain::identity::TenantRepository for TenantRepository<'a> {
    async fn add(&self, tenant: &Tenant) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let tenant_id: Uuid = tenant.tenant_id().into();
        let tenant_name: String = tenant.name().into();
        let tenant_description: Option<String> = tenant.description().as_ref().map(|td| td.into());
        let tenant_enabled = tenant.active();
        query_file!(
            "sql/postgres/insert_tenant.sql",
            tenant_id,
            tenant_name,
            tenant_description,
            tenant_enabled,
        )
        .execute(&mut *tx)
        .await?;
        for invitation in tenant.invitations() {
            let invitation_id: String = invitation.invitation_id().into();
            let invitation_description: String = invitation.description().into();
            let valid_from = invitation.validity().start_date();
            let until = invitation.validity().end_date();
            query_file!(
                "sql/postgres/insert_invitation.sql",
                tenant_id,
                invitation_id,
                invitation_description,
                valid_from,
                until,
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn update(&self, tenant: &Tenant) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        let tenant_id: Uuid = tenant.tenant_id().into();
        let tenant_name: String = tenant.name().into();
        let tenant_description: Option<String> = tenant.description().as_ref().map(|td| td.into());
        let tenant_enabled = tenant.active();
        query_file!(
            "sql/postgres/update_tenant.sql",
            tenant_id,
            tenant_name,
            tenant_description,
            tenant_enabled,
        )
        .execute(&mut *tx)
        .await?;
        for invitation in tenant.invitations() {
            let invitation_id: String = invitation.invitation_id().into();
            let invitation_description: String = invitation.description().into();
            let valid_from = invitation.validity().start_date();
            let until = invitation.validity().end_date();
            query_file!(
                "sql/postgres/insert_invitation.sql",
                tenant_id,
                invitation_id,
                invitation_description,
                valid_from,
                until,
            )
                .execute(&mut *tx)
                .await?;
        }
        // Delete invitations not in tenant anymore.
        let invitation_ids: Vec<String> = tenant.invitations().iter()
            .map(|ri| ri.invitation_id().into())
            .collect();
        query_file!("sql/postgres/delete_invitations.sql", tenant_id, &invitation_ids)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(())
    }

    async fn remove(&self, tenant: &Tenant) -> Result<()> {
        let tenant_id: Uuid = tenant.tenant_id().into();
        query_file!("sql/postgres/delete_tenant.sql", tenant_id)
            .execute(self.pool)
            .await?;
        Ok(())
    }

    async fn find_by_name(&self, name: &TenantName) -> Result<Tenant> {
        let tenant_name: String = name.into();
        let rows = query_file_as!(
            TenantAndInvitationRow,
            "sql/postgres/find_tenant_by_name.sql",
            tenant_name
        )
        .fetch_all(self.pool)
        .await?;
        rows.try_into()
    }

    async fn find_by_id(&self, id: &TenantId) -> Result<Tenant> {
        let tenant_id: Uuid = id.into();
        let rows = query_file_as!(
            TenantAndInvitationRow,
            "sql/postgres/find_tenant_by_id.sql",
            tenant_id
        )
        .fetch_all(self.pool)
        .await?;
        rows.try_into()
    }
}

#[derive(sqlx::FromRow)]
struct TenantAndInvitationRow {
    tenant_id: Uuid,
    tenant_name: String,
    tenant_description: Option<String>,
    tenant_enabled: bool,
    invitation_id: String,
    invitation_description: String,
    valid_from: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
}
impl TryInto<RegistrationInvitation> for TenantAndInvitationRow {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RegistrationInvitation> {
        let invitation_id: InvitationId = self.invitation_id.try_into()?;
        let invitation_description: InvitationDescription =
            self.invitation_description.try_into()?;
        let validity = Validity::new(self.valid_from, self.until)?;
        Ok(RegistrationInvitation::hydrate(
            invitation_id,
            invitation_description,
            validity,
        ))
    }
}

impl TryInto<Tenant> for Vec<TenantAndInvitationRow> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Tenant> {
        let first_row = self.get(0).ok_or(anyhow!("no rows available"))?;
        let tenant_id: TenantId = first_row.tenant_id.into();
        let tenant_name: TenantName = TenantName::new(&first_row.tenant_name)?;
        let tenant_description = match first_row.tenant_description.clone() {
            Some(description) => Some(TenantDescription::new(&description)?),
            None => None,
        };
        let tenant_enabled = first_row.tenant_enabled;
        let mut invitations: Vec<RegistrationInvitation> = Vec::new();
        for row in self {
            invitations.push(row.try_into()?);
        }
        Ok(Tenant::hydrate(
            tenant_id,
            tenant_name,
            tenant_description,
            tenant_enabled,
            invitations,
        ))
    }
}
