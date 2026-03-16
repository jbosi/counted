use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{AccountProject, UpsertAccountProject};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::account_projects::account_projects_repository;
#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use crate::utils::get_current_account_id;

#[get("/api/v1/account/projects")]
pub async fn get_account_projects() -> Result<Vec<AccountProject>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let Some(account_id) = get_current_account_id().await else {
            return Err(ServerFnError::new("Forbidden"));
        };

        let pool = get_db().await;
        let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

        let projects = account_projects_repository::get_account_projects(&mut *tx, account_id).await?;

        tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(projects)
    }
    #[cfg(not(feature = "server"))]
    Ok(vec![])
}

#[post("/api/v1/account/projects")]
pub async fn upsert_account_project(
    Json(payload): Json<UpsertAccountProject>,
) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        let Some(account_id) = get_current_account_id().await else {
            return Err(ServerFnError::new("Forbidden"));
        };

        let pool = get_db().await;
        let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

        account_projects_repository::upsert_account_project(
            &mut *tx,
            account_id,
            payload.project_id,
            payload.user_id,
        )
        .await?;

        tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    }
    Ok(())
}

#[post("/api/v1/account/projects/batch")]
pub async fn batch_upsert_account_projects(
    Json(payload): Json<Vec<UpsertAccountProject>>,
) -> Result<Vec<Uuid>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let Some(account_id) = get_current_account_id().await else {
            return Err(ServerFnError::new("Forbidden"));
        };

        let pool = get_db().await;
        let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

        let accepted = account_projects_repository::batch_upsert_account_projects(
            &mut *tx,
            account_id,
            payload,
        )
        .await?;

        tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

        return Ok(accepted);
    }
    #[cfg(not(feature = "server"))]
    Ok(vec![])
}

#[delete("/api/v1/account/projects/{project_id}")]
pub async fn delete_account_project(project_id: Uuid) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        let Some(account_id) = get_current_account_id().await else {
            return Err(ServerFnError::new("Forbidden"));
        };

        let pool = get_db().await;
        let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

        account_projects_repository::delete_account_project(&mut *tx, account_id, project_id)
            .await?;

        tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    }
    Ok(())
}
