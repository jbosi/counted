use dioxus::prelude::*;
use shared::{AccountProject, UpsertAccountProject};
use uuid::Uuid;

#[cfg(feature = "server")]
use sqlx::{PgConnection, Postgres, QueryBuilder};

#[cfg(feature = "server")]
pub async fn get_account_projects(
    executor: &mut PgConnection,
    account_id: Uuid,
) -> Result<Vec<AccountProject>, ServerFnError> {
    let rows = sqlx::query_as!(
        AccountProject,
        "SELECT project_id, user_id FROM account_projects WHERE account_id = $1",
        account_id
    )
    .fetch_all(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to get account projects: {}", e)))?;

    Ok(rows)
}

#[cfg(feature = "server")]
pub async fn upsert_account_project(
    executor: &mut PgConnection,
    account_id: Uuid,
    project_id: Uuid,
    user_id: Option<i32>,
) -> Result<(), ServerFnError> {
    sqlx::query!(
        "INSERT INTO account_projects (account_id, project_id, user_id)
         VALUES ($1, $2, $3)
         ON CONFLICT (account_id, project_id) DO UPDATE SET user_id = EXCLUDED.user_id",
        account_id,
        project_id,
        user_id
    )
    .execute(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to upsert account project: {}", e)))?;

    Ok(())
}

/// Batch-upserts account_projects entries. Silently skips any project_id that does not exist
/// (avoids FK violations from stale localStorage entries). Returns the accepted project IDs.
#[cfg(feature = "server")]
pub async fn batch_upsert_account_projects(
    executor: &mut PgConnection,
    account_id: Uuid,
    entries: Vec<UpsertAccountProject>,
) -> Result<Vec<Uuid>, ServerFnError> {
    if entries.is_empty() {
        return Ok(vec![]);
    }

    let requested_ids: Vec<Uuid> = entries.iter().map(|e| e.project_id).collect();
    let valid_ids: Vec<Uuid> = sqlx::query_scalar("SELECT id FROM projects WHERE id = ANY($1)")
        .bind(&requested_ids)
        .fetch_all(&mut *executor)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to validate project IDs: {}", e)))?;

    let valid_set: std::collections::HashSet<Uuid> = valid_ids.iter().copied().collect();
    let valid_entries: Vec<&UpsertAccountProject> =
        entries.iter().filter(|e| valid_set.contains(&e.project_id)).collect();

    if valid_entries.is_empty() {
        return Ok(vec![]);
    }

    let mut qb: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO account_projects (account_id, project_id, user_id) ");
    qb.push_values(valid_entries.iter(), |mut b, e| {
        b.push_bind(account_id).push_bind(e.project_id).push_bind(e.user_id);
    });
    qb.push(" ON CONFLICT (account_id, project_id) DO UPDATE SET user_id = EXCLUDED.user_id");

    qb.build()
        .execute(&mut *executor)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to batch upsert account projects: {}", e)))?;

    Ok(valid_ids)
}

#[cfg(feature = "server")]
pub async fn delete_account_project(
    executor: &mut PgConnection,
    account_id: Uuid,
    project_id: Uuid,
) -> Result<(), ServerFnError> {
    sqlx::query!(
        "DELETE FROM account_projects WHERE account_id = $1 AND project_id = $2",
        account_id,
        project_id
    )
    .execute(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to delete account project: {}", e)))?;

    Ok(())
}
