use crate::domain::identity::{InvitationDescription, InvitationId, RegistrationInvitation, Tenant, TenantDescription, TenantId, TenantName, TenantRepositoryError, Validity};
use crate::ports::adapters::postgres::{invitation, tenant};
use anyhow::Result;
use sqlx::Pool;

/// Implementation of TenantRepository for PostgreSQL
pub struct TenantRepository<'a> {
    pool: &'a Pool<sqlx::Postgres>,
}

impl<'a> TenantRepository<'a> {
    pub fn new(pool: &'a Pool<sqlx::Postgres>) -> Self {
        TenantRepository { pool }
    }
}

impl<'a> crate::domain::identity::TenantRepository for TenantRepository<'a> {
    async fn add(&self, tenant: Tenant) -> Result<Tenant> {
        let mut tx = self.pool.begin().await?;
        let row = tenant::insert(&mut *tx, &(&tenant).into()).await?;
        let invitations = tenant
            .invitations()
            .into_iter()
            .map(|inv| inv.into())
            .collect::<Vec<invitation::Row>>();
        let invitations_rows = invitation::insert_all(&mut *tx, row.id, invitations).await?;
        tx.commit().await?;
        to_tenant(row, invitations_rows)
    }

    async fn update(&self, tenant: Tenant) -> Result<Tenant> {
        let mut tx = self.pool.begin().await?;
        let row = tenant::update(&mut *tx, &(&tenant).into()).await?;
        let invitations = tenant
            .invitations()
            .into_iter()
            .map(|inv| inv.into())
            .collect::<Vec<invitation::Row>>();
        let invitations_rows =
            invitation::update_invitations(&mut *tx, row.id, invitations).await?;
        tx.commit().await?;
        to_tenant(row, invitations_rows)
    }

    async fn remove(&self, tenant: Tenant) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        invitation::delete_all(&mut *tx, tenant.id().unwrap()).await?;
        tenant::delete_by_id(&mut *tx, tenant.id().unwrap(), tenant.version()).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn find_by_name(&self, name: &str) -> Result<Tenant> {
        let tenant_row = tenant::load_by_name(self.pool, name.as_ref()).await?;
        match tenant_row {
            Some(row) => {
                let invitations_rows = invitation::load_all(self.pool, row.id).await?;
                to_tenant(row, invitations_rows)
            }
            None => Err(TenantRepositoryError::NameNotFound(name.into()).into()),
        }
    }

    async fn find_by_id(&self, id: &TenantId) -> Result<Tenant> {
        let tenant_row = tenant::load_by_id(self.pool, id.clone().into()).await?;
        match tenant_row {
            Some(row) => {
                let invitations_rows = invitation::load_all(self.pool, row.id).await?;
                to_tenant(row, invitations_rows)
            }
            None => Err(TenantRepositoryError::NotFound(id.clone()).into()),
        }
    }
}

fn to_tenant(row: tenant::Row, invitations_rows: Vec<invitation::Row>) -> Result<Tenant> {
    let invitations = to_invitations(invitations_rows)?;
    let tenant_id: TenantId = row.uuid.try_into()?;
    Ok(Tenant::hydrate(
        row.id,
        row.version,
        tenant_id,
        TenantName::new(&row.name)?,
        if let Some(d) = row.description {
            Some(TenantDescription::new(&d)?)
        } else {
            None
        },
        row.enabled,
        invitations,
        row.created_at,
        row.updated_at,
    ))
}

fn to_invitations(rows: Vec<invitation::Row>) -> Result<Vec<RegistrationInvitation>> {
    rows.into_iter()
        .map(to_invitation)
        .collect::<Result<Vec<RegistrationInvitation>>>()
}

fn to_invitation(row: invitation::Row) -> Result<RegistrationInvitation> {
    let validity = Validity::new(row.valid_from, row.until)?;
    Ok(RegistrationInvitation::hydrate(
        row.id,
        InvitationId::new(&row.identifier)?,
        InvitationDescription::new(&row.description)?,
        validity,
    ))
}