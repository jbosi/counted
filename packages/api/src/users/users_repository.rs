use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature = "server")]
use anyhow::Context;
use shared::UserProjects;
use shared::{CreatableUser, User};
#[cfg(feature = "server")]
use sqlx::{PgConnection, Postgres, QueryBuilder};

#[cfg(feature = "server")]
pub async fn get_users(executor: &mut PgConnection) -> Result<Vec<User>, ServerFnError> {
    let users: Vec<User> = sqlx::query_as!(User, "SELECT id, name, balance, created_at FROM users")
        .fetch_all(&mut *executor)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

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
    // TODO: handle different project_ids or force only one project_id
    if creatable_users.is_empty() {
        return Ok(vec![]);
        // Err(ServerFnError::new("No users supplied"))
    }

    let project_id = creatable_users[0].project_id;

    // ADD into user table
    let mut users_query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO users (name) ");

    users_query_builder
        .push_values(creatable_users, |mut query_builder, user| {
            query_builder.push_bind(user.name);
        })
        .push(" RETURNING id, name, balance, created_at");

    let users_query = users_query_builder.build_query_as::<User>();

    let users: Vec<User> = users_query
        .fetch_all(&mut *executor)
        .await
        .context("Failed to add users")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // ADD into user_projects table
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
    .map_err(|e| ServerFnError::new(e.to_string()))?;

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
            .map_err(|e| ServerFnError::new(e.to_string()))?;

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
