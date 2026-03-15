use chrono::NaiveDateTime;
use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature = "server")]
use sqlx::PgConnection;
use shared::Account;

#[cfg(feature = "server")]
#[derive(sqlx::FromRow)]
pub struct AccountWithHash {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub created_at: NaiveDateTime,
    pub password_hash: String,
    pub failed_login_count: i32,
    pub locked_until: Option<NaiveDateTime>,
}

#[cfg(feature = "server")]
pub async fn create_account(
    executor: &mut PgConnection,
    email: &str,
    password_hash: &str,
    display_name: &str,
) -> Result<Uuid, ServerFnError> {
    let id: Uuid = sqlx::query_scalar!(
        "INSERT INTO accounts (email, password_hash, display_name) VALUES ($1, $2, $3) RETURNING id",
        email,
        password_hash,
        display_name
    )
    .fetch_one(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to create account: {}", e)))?;

    Ok(id)
}

#[cfg(feature = "server")]
pub async fn find_account_by_email(
    executor: &mut PgConnection,
    email: &str,
) -> Result<Option<AccountWithHash>, ServerFnError> {
    let account = sqlx::query_as!(
        AccountWithHash,
        "SELECT id, email, display_name, created_at, password_hash, failed_login_count, locked_until FROM accounts WHERE email = $1",
        email
    )
    .fetch_optional(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to find account by email: {}", e)))?;

    Ok(account)
}

#[cfg(feature = "server")]
pub async fn get_account_by_id(
    executor: &mut PgConnection,
    id: Uuid,
) -> Result<Option<Account>, ServerFnError> {
    let account = sqlx::query_as!(
        Account,
        "SELECT id, email, display_name, created_at FROM accounts WHERE id = $1",
        id
    )
    .fetch_optional(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to get account by id: {}", e)))?;

    Ok(account)
}

#[cfg(feature = "server")]
pub async fn increment_failed_login(
    executor: &mut PgConnection,
    id: Uuid,
) -> Result<(), ServerFnError> {
    sqlx::query!(
        "UPDATE accounts
         SET
           failed_login_count = failed_login_count + 1,
           locked_until = CASE
             WHEN failed_login_count + 1 >= 5 THEN NOW() + INTERVAL '15 minutes'
             ELSE locked_until
           END
         WHERE id = $1",
        id
    )
    .execute(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to increment failed login count: {}", e)))?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn reset_failed_login(
    executor: &mut PgConnection,
    id: Uuid,
) -> Result<(), ServerFnError> {
    sqlx::query!(
        "UPDATE accounts SET failed_login_count = 0, locked_until = NULL WHERE id = $1",
        id
    )
    .execute(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to reset failed login count: {}", e)))?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn create_session(
    executor: &mut PgConnection,
    account_id: Uuid,
    expires_at: NaiveDateTime,
) -> Result<Uuid, ServerFnError> {
    let session_id: Uuid = sqlx::query_scalar!(
        "INSERT INTO sessions (account_id, expires_at) VALUES ($1, $2) RETURNING id",
        account_id,
        expires_at
    )
    .fetch_one(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to create session: {}", e)))?;

    Ok(session_id)
}

#[cfg(feature = "server")]
pub async fn get_session_account_id(
    executor: &mut PgConnection,
    session_id: Uuid,
) -> Result<Option<Uuid>, ServerFnError> {
    let account_id: Option<Uuid> = sqlx::query_scalar!(
        "SELECT account_id FROM sessions WHERE id = $1 AND expires_at > NOW()",
        session_id
    )
    .fetch_optional(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to get session: {}", e)))?;

    Ok(account_id)
}

#[cfg(feature = "server")]
pub async fn delete_session(
    executor: &mut PgConnection,
    session_id: Uuid,
) -> Result<(), ServerFnError> {
    sqlx::query!("DELETE FROM sessions WHERE id = $1", session_id)
        .execute(&mut *executor)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to delete session: {}", e)))?;

    Ok(())
}
