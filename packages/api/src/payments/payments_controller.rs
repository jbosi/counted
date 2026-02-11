use dioxus::prelude::*;
use uuid::Uuid;

use crate::payments::payments_repository;
use shared::{Payment, UserSummary};

#[get("/api/expenses/{expense_id}/payments")]
pub async fn get_payments_by_expense_id(expense_id: i32) -> Result<Vec<Payment>, ServerFnError> {
    let payments = payments_repository::get_payments_by_expense_id(expense_id).await?;

    Ok(payments)
}

#[server()]
pub async fn get_payments_by_user_id(user_id: i32) -> Result<Vec<Payment>, ServerFnError> {
    let payments = payments_repository::get_payments_by_user_id(user_id).await?;

    Ok(payments)
}

#[get("/api/projects/{project_id}/expenses/summary")]
pub async fn get_summary_by_project_id(project_id: Uuid) -> Result<UserSummary, ServerFnError> {
    // Delegate to the repository function which has the correct logic for handling different expense types
    payments_repository::get_summary_by_project_id(project_id).await
}
