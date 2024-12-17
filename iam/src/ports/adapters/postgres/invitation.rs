use crate::domain::identity::RegistrationInvitation;
use chrono::{DateTime, Utc};
use sqlx::{query, query_as, Executor, Postgres};

#[derive(sqlx::FromRow, Debug, PartialEq)]
pub struct Row {
    pub id: i32,
    pub tenant_id: i32,
    pub identifier: String,
    pub description: String,
    pub valid_from: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

impl From<&RegistrationInvitation> for Row {
    fn from(invitation: &RegistrationInvitation) -> Self {
        Row {
            id: invitation.id().unwrap_or(i32::default()),
            tenant_id: i32::default(),
            identifier: invitation.invitation_id().clone().into_string(),
            description: invitation.description().clone().into_string(),
            valid_from: invitation.validity().start_date().map(|s| s.clone()),
            until: invitation.validity().end_date().map(|e| e.clone()),
        }
    }
}

pub async fn insert_all<E>(
    executor: &mut E,
    tenant_id: i32,
    invitations: Vec<Row>,
) -> Result<Vec<Row>, sqlx::Error>
where
    for<'e> &'e mut E: Executor<'e, Database = Postgres>,
{
    let mut results = Vec::new();
    for row in invitations {
        results.push(insert(executor, tenant_id, &row).await?);
    }
    Ok(results)
}

pub async fn insert<E>(executor: &mut E, tenant_id: i32, row: &Row) -> Result<Row, sqlx::Error>
where
    for<'e> &'e mut E: Executor<'e, Database = Postgres>,
{
    let query = query_as!(
        Row,
        r#"
        INSERT
          INTO invitation (tenant_id, identifier, description, valid_from, until)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        tenant_id,
        row.identifier,
        row.description,
        row.valid_from,
        row.until
    );
    query.fetch_one(executor).await
}

pub async fn update_invitations<E>(
    executor: &mut E,
    tenant_id: i32,
    invitations: Vec<Row>,
) -> Result<Vec<Row>, sqlx::Error>
where
    for<'e> &'e mut E: Executor<'e, Database = Postgres>,
{
    // Load current rows.
    let current_rows = load_all(&mut *executor, tenant_id).await?;
    // Find rows to be inserted
    let mut result_rows: Vec<Row> = Vec::new();
    for row in invitations {
        if row.id == i32::default() {
            let result_row = insert(executor, tenant_id, &row).await?;
            result_rows.push(result_row);
        } else if current_rows.contains(&row) {
            let result_row = update(executor, &row).await?;
            result_rows.push(result_row);
        } else {
            let _ = delete(executor, row.id).await?;
        }
    }
    Ok(result_rows)
}

pub async fn update<E>(executor: &mut E, row: &Row) -> Result<Row, sqlx::Error>
where
    for<'e> &'e mut E: Executor<'e, Database = Postgres>,
{
    let query = query_as!(
        Row,
        r#"
        UPDATE invitation
           SET identifier = $2,
               description = $3,
               valid_from = $4,
               until = $5
         WHERE id = $1
        RETURNING *
        "#,
        row.id,
        row.identifier,
        row.description,
        row.valid_from,
        row.until
    );
    query.fetch_one(executor).await
}

pub async fn delete<E>(executor: &mut E, id: i32) -> Result<Row, sqlx::Error>
where
    for<'e> &'e mut E: Executor<'e, Database = Postgres>,
{
    let query = query_as!(
        Row,
        r#"
        DELETE
          FROM invitation
         WHERE id = $1
        RETURNING *
        "#,
        id
    );
    query.fetch_one(executor).await
}

pub async fn delete_all<E>(executor: &mut E, tenant_id: i32) -> Result<(), sqlx::Error>
where
    for<'e> &'e mut E: Executor<'e, Database = Postgres>,
{
    let query = query!(
        r#"
        DELETE
          FROM invitation
         WHERE tenant_id = $1
        "#,
        tenant_id
    );
    query.execute(executor).await.map(|_| ())
}

pub async fn load_all<'e, E>(executor: E, tenant_id: i32) -> Result<Vec<Row>, sqlx::Error>
where
    E: Executor<'e, Database = Postgres>, //    for<'e> &'e mut E: Executor<'e, Database = Postgres>
{
    let query = query_as!(
        Row,
        r#"
        SELECT *
          FROM invitation
         WHERE tenant_id = $1
        "#,
        tenant_id
    );
    query.fetch_all(executor).await
}
