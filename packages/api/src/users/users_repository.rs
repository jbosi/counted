use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature = "server")]
use anyhow::Context;
use shared::UserProjects;
use shared::{CreatableUser, User};
#[cfg(feature = "server")]
use sqlx::{PgConnection, Postgres, QueryBuilder};

#[cfg(feature = "server")]
use crate::account_projects::account_projects_repository;
#[cfg(feature = "server")]
use crate::utils::sha256_hex;

#[cfg(feature = "server")]
pub async fn get_users(executor: &mut PgConnection) -> Result<Vec<User>, ServerFnError> {
    let users: Vec<User> = sqlx::query_as!(User, "SELECT id, name, balance, created_at FROM users")
        .fetch_all(&mut *executor)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to get users: {}", e)))?;

    Ok(users)
}

#[cfg(feature = "server")]
pub async fn delete_user(executor: &mut PgConnection, user_id: i32) -> Result<(), ServerFnError> {
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&mut *executor)
        .await
        .context("Failed to delete user in user table with specified id")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn delete_users(
    executor: &mut PgConnection,
    user_ids: Vec<i32>,
) -> Result<(), ServerFnError> {
    sqlx::query!("DELETE FROM users WHERE id = ANY($1)", &user_ids[..])
        .execute(&mut *executor)
        .await
        .context("Failed to delete user in user table with specified id")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn add_users(
    executor: &mut PgConnection,
    creatable_users: Vec<CreatableUser>,
) -> Result<Vec<User>, ServerFnError> {
    if creatable_users.is_empty() {
        return Ok(vec![]);
    }

    let project_id = creatable_users[0].project_id;

    // Collect email data before QueryBuilder consumes ownership
    let email_hashes: Vec<Option<String>> = creatable_users
        .iter()
        .map(|u| u.invited_email.as_ref().map(|e| sha256_hex(&e.to_lowercase())))
        .collect();
    let emails_lower: Vec<Option<String>> = creatable_users
        .iter()
        .map(|u| u.invited_email.as_ref().map(|e| e.to_lowercase()))
        .collect();

    // INSERT into users table with email_hash
    let mut users_query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO users (name, email_hash) ");

    users_query_builder
        .push_values(creatable_users.iter(), |mut qb, user| {
            let hash = user.invited_email.as_ref().map(|e| sha256_hex(&e.to_lowercase()));
            qb.push_bind(user.name.clone()).push_bind(hash);
        })
        .push(" RETURNING id, name, balance, created_at");

    let users_query = users_query_builder.build_query_as::<User>();

    let users: Vec<User> = users_query
        .fetch_all(&mut *executor)
        .await
        .context("Failed to add users")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // INSERT into user_projects table
    let mut user_projects_query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO user_projects(user_id, project_id) ");

    user_projects_query_builder.push_values(users.clone(), |mut query_builder, user| {
        query_builder.push_bind(user.id).push_bind(project_id);
    });

    let user_projects_query = user_projects_query_builder.build();

    user_projects_query
        .execute(&mut *executor)
        .await
        .context("Failed to associate user with project")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // For each invited email: immediate account lookup + name update + account_projects upsert
    for ((user, hash_opt), email_opt) in users.iter().zip(email_hashes.iter()).zip(emails_lower.iter()) {
        if hash_opt.is_some() {
            if let Some(ref email) = email_opt {
                let account: Option<(Uuid, String)> =
                    sqlx::query_as("SELECT id, display_name FROM accounts WHERE lower(email) = $1")
                        .bind(email)
                        .fetch_optional(&mut *executor)
                        .await
                        .map_err(|e| ServerFnError::new(format!("Failed to lookup account: {}", e)))?;

                if let Some((account_id, display_name)) = account {
                    sqlx::query!("UPDATE users SET name = $1 WHERE id = $2", display_name, user.id)
                        .execute(&mut *executor)
                        .await
                        .map_err(|e| ServerFnError::new(format!("Failed to update user name: {}", e)))?;

                    account_projects_repository::upsert_account_project(
                        &mut *executor,
                        account_id,
                        project_id,
                        Some(user.id),
                    )
                    .await?;
                }
                // Silent if not found — no error, no enumeration
            }
        }
    }

    Ok(users)
}

#[cfg(feature = "server")]
pub async fn add_user(executor: &mut PgConnection, user: CreatableUser) -> Result<i32, ServerFnError> {
    let user_id: i32 =
        sqlx::query_scalar!("INSERT INTO users(name) VALUES ($1) RETURNING id", user.name)
            .fetch_one(&mut *executor)
            .await
            .context("Failed to insert user into database")
            .map_err(|e| ServerFnError::new(e.to_string()))?;

    sqlx::query!(
        "INSERT INTO user_projects(user_id, project_id) VALUES ($1, $2)",
        user_id,
        user.project_id
    )
    .execute(&mut *executor)
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to associate user with project: {}", e)))?;

    Ok(user_id)
}

#[cfg(feature = "server")]
pub async fn get_users_by_project_id(
    executor: &mut PgConnection,
    project_id: Uuid,
) -> Result<Vec<User>, ServerFnError> {
    let user_ids: Vec<i32> =
        sqlx::query_scalar!("SELECT user_id FROM user_projects WHERE project_id = $1", project_id)
            .fetch_all(&mut *executor)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to get user ids by project: {}", e)))?;

    if user_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT id, name, balance, created_at FROM users WHERE id IN (");

    let mut separated = query_builder.separated(", ");
    for id in user_ids {
        separated.push_bind(id);
    }
    separated.push_unseparated(")");

    let query = query_builder.build_query_as::<User>();
    let users: Vec<User> = query
        .fetch_all(&mut *executor)
        .await
        .context("Failed to fetch users by IDs")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(users)
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use crate::utils::sha256_hex;

    #[test]
    fn sha256_is_deterministic() {
        assert_eq!(sha256_hex("alice@example.com"), sha256_hex("alice@example.com"));
    }

    #[test]
    fn sha256_different_inputs_differ() {
        assert_ne!(sha256_hex("alice@example.com"), sha256_hex("bob@example.com"));
    }

    #[test]
    fn sha256_case_sensitivity() {
        // Callers must lowercase before calling — different cases produce different hashes
        assert_ne!(sha256_hex("Alice@Example.com"), sha256_hex("alice@example.com"));
    }

    #[test]
    fn sha256_output_format() {
        let hash = sha256_hex("alice@example.com");
        assert_eq!(hash.len(), 64); // 32 bytes = 64 hex chars
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
