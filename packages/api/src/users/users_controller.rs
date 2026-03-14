use dioxus::{fullstack::Json, prelude::*};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use crate::payments::payments_repository::get_payments_by_user_id;
use crate::users::users_repository;
use shared::{CreatableUser, CreatableUserBatch, User};

#[get("/api/v1/users")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let users = users_repository::get_users(&mut *tx).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(users)
}

#[delete("/api/v1/users/{user_id}")]
pub async fn delete_user(user_id: i32) -> Result<(), ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let payments = get_payments_by_user_id(&mut *tx, user_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if !payments.is_empty() {
        // User still has payments → abort with a clear error
        return Err(ServerFnError::new(
            "User has existing payments in this project and cannot be removed",
        ));
    }

    users_repository::delete_user(&mut *tx, user_id).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[post("/api/v1/users")]
pub async fn add_user(Json(payload): Json<CreatableUserBatch>) -> Result<Vec<User>, ServerFnError> {
    let users: Vec<CreatableUser> = match payload {
        CreatableUserBatch::Single(u) => vec![u],
        CreatableUserBatch::Multiple(v) => v,
    };

    const MAX_BATCH: usize = 100;
    if users.len() > MAX_BATCH {
        return Err(ServerFnError::new(format!("Batch size exceeds {}", MAX_BATCH)));
    }

    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let users = users_repository::add_users(&mut *tx, users.clone()).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(users)
}

#[get("/api/v1/projects/{project_id}/users")]
pub async fn get_users_by_project_id(project_id: Uuid) -> Result<Vec<User>, ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let users = users_repository::get_users_by_project_id(&mut *tx, project_id).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(users)
}
