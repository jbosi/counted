use dioxus::{fullstack::Json, prelude::*};
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::payments::payments_repository::get_payments_by_user_id;
use crate::users::users_repository;
use shared::{CreatableUser, User};

#[get("/api/users")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let users: Vec<User> = users_repository::get_users().await?;

    Ok(users)
}

#[delete("/api/users/{user_id}")]
pub async fn delete_users(user_id: i32) -> Result<(), ServerFnError> {
    let payments =
        get_payments_by_user_id(user_id).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    if !payments.is_empty() {
        // User still has payments → abort with a clear error
        return Err(ServerFnError::new(
            "User has existing payments in this project and cannot be removed",
        ));
    }

    users_repository::delete_users(user_id).await?;

    Ok(())
}

#[post("/api/users")]
pub async fn add_user(Json(user): Json<CreatableUser>) -> Result<User, ServerFnError> {
    let user_id = users_repository::add_user(user.clone()).await?;

    let created_user = User { id: user_id, name: user.name, balance: None, created_at: None };

    Ok(created_user)
}

#[get("/api/projects/{project_id}/users")]
pub async fn get_users_by_project_id(project_id: Uuid) -> Result<Vec<User>, ServerFnError> {
    let users = users_repository::get_users_by_project_id(project_id).await?;

    Ok(users)
}
