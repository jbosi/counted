use chrono::NaiveDateTime;
use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use anyhow::Context;
#[cfg(feature = "server")]
use sqlx::{Pool, Postgres};
use shared::Account;

#[cfg(feature = "server")]
pub struct AccountWithHash {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub created_at: NaiveDateTime,
    pub password_hash: String,
}

#[cfg(feature = "server")]
pub async fn create_account(
    email: &str,
    password_hash: &str,
    display_name: &str,
) -> Result<Uuid, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO accounts (email, password_hash, display_name) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .fetch_one(&pool)
    .await
    .context("Failed to create account")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(id)
}

#[cfg(feature = "server")]
pub async fn find_account_by_email(email: &str) -> Result<Option<AccountWithHash>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let row = sqlx::query_as::<_, (Uuid, String, String, NaiveDateTime, String)>(
        "SELECT id, email, display_name, created_at, password_hash FROM accounts WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(&pool)
    .await
    .context("Failed to query account by email")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(row.map(|(id, email, display_name, created_at, password_hash)| AccountWithHash {
        id,
        email,
        display_name,
        created_at,
        password_hash,
    }))
}

#[cfg(feature = "server")]
pub async fn get_account_by_id(id: Uuid) -> Result<Option<Account>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let account = sqlx::query_as::<_, Account>(
        "SELECT id, email, display_name, created_at FROM accounts WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .context("Failed to query account by id")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(account)
}

#[cfg(feature = "server")]
pub async fn create_session(account_id: Uuid, expires_at: NaiveDateTime) -> Result<Uuid, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let session_id: Uuid = sqlx::query_scalar(
        "INSERT INTO sessions (account_id, expires_at) VALUES ($1, $2) RETURNING id",
    )
    .bind(account_id)
    .bind(expires_at)
    .fetch_one(&pool)
    .await
    .context("Failed to create session")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(session_id)
}

#[cfg(feature = "server")]
pub async fn get_session_account_id(session_id: Uuid) -> Result<Option<Uuid>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let account_id: Option<Uuid> = sqlx::query_scalar(
        "SELECT account_id FROM sessions WHERE id = $1 AND expires_at > NOW()",
    )
    .bind(session_id)
    .fetch_optional(&pool)
    .await
    .context("Failed to query session")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(account_id)
}

#[cfg(feature = "server")]
pub async fn delete_session(session_id: Uuid) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    sqlx::query("DELETE FROM sessions WHERE id = $1")
        .bind(session_id)
        .execute(&pool)
        .await
        .context("Failed to delete session")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
