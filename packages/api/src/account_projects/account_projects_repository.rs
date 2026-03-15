use dioxus::prelude::*;
use shared::AccountProject;
use uuid::Uuid;

#[cfg(feature = "server")]
use sqlx::PgConnection;

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
