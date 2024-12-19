use crate::domain::identity::Tenant;
use chrono::{DateTime, Utc};
use sqlx::Error::RowNotFound;
use sqlx::{query, query_as, Executor, Postgres};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct Row {
    pub id: i32,
    pub version: i32,
    pub uuid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&Tenant> for Row {
    fn from(tenant: &Tenant) -> Self {
        Row {
            id: i32::default(),
            version: i32::default(),
            uuid: tenant.tenant_id().clone().into(),
            name: tenant.name().clone().into(),
            description: tenant.description().clone().map(|s| s.into()),
            enabled: tenant.active(),
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        }
    }
}

pub async fn insert<E>(executor: &mut E, row: &Row) -> Result<Row, sqlx::Error>
where
    for<'e> &'e mut E: Executor<'e, Database = Postgres>,
{
    let query = query_as!(
        Row,
        r#"
            INSERT
              INTO tenant (uuid, name, description, enabled, created_at, updated_at)
            VALUES ($1, $2, $3, $4, current_timestamp, current_timestamp)
            RETURNING *
        "#,
        row.uuid,
        row.name,
        row.description,
        row.enabled
    );
    query.fetch_one(executor).await
}

pub async fn update<E>(executor: &mut E, row: &Row) -> Result<Row, sqlx::Error>
where
    for<'e> &'e mut E: Executor<'e, Database = Postgres>,
{
    let query = query_as!(
        Row,
        r#"
        UPDATE tenant
           SET version = version + 1,
               name = $2,
               description = $3,
               enabled = $4,
               updated_at = current_timestamp
         WHERE uuid = $1
        RETURNING *
        "#,
        row.uuid,
        row.name,
        row.description,
        row.enabled
    );
    query.fetch_one(executor).await
}

pub async fn delete_by_id<E>(executor: &mut E, tenant_id: &Uuid) -> Result<(), sqlx::Error>
where
    for<'e> &'e mut E: Executor<'e, Database = Postgres>
{
    let query = query!(
        r#"
        DELETE
          FROM tenant
        WHERE uuid = $1
        "#,
        tenant_id
    );
    query.execute(executor).await.map(|_| ())
}

pub async fn load_by_id<'e, E>(executor: E, tenant_id: &Uuid) -> Result<Option<Row>, sqlx::Error>
where E: Executor<'e, Database=Postgres>
{
    let query = query_as!(
        Row,
        r#"
        SELECT *
          FROM tenant
         WHERE uuid = $1
        "#,
        tenant_id
    );
    match query.fetch_one(executor).await {
        Ok(row) => Ok(Some(row)),
        Err(RowNotFound) => Ok(None),
        Err(err) => Err(err.into()),
    }
}

pub async fn load_by_name<'e, E>(executor: E, name: &str) -> Result<Option<Row>, sqlx::Error>
where E: Executor<'e, Database=Postgres>
//    for<'e> &'e mut E: Executor<'e, Database = Postgres>
{
    let query = query_as!(
        Row,
        r#"
        SELECT *
          FROM tenant
         WHERE name = $1
        "#,
        name
    );
    match query.fetch_one(executor).await {
        Ok(row) => Ok(Some(row)),
        Err(RowNotFound) => Ok(None),
        Err(err) => Err(err.into()),
    }
}
