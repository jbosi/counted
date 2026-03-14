use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
use crate::expenses::expenses_repository;
use crate::payments::payments_repository;
use shared::{Payment, UserSummary};

#[get("/api/v1/expenses/{expense_id}/payments")]
pub async fn get_payments_by_expense_id(expense_id: i32) -> Result<Vec<Payment>, ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let payments = payments_repository::get_payments_by_expense_id(&mut *tx, expense_id).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(payments)
}

#[server()]
pub async fn get_payments_by_user_id(user_id: i32) -> Result<Vec<Payment>, ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let payments = payments_repository::get_payments_by_user_id(&mut *tx, user_id).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(payments)
}

#[get("/api/v1/projects/{project_id}/payments")]
pub async fn get_payments_by_project_id(project_id: Uuid) -> Result<Vec<Payment>, ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let expenses = expenses_repository::get_expenses_by_project_id(&mut *tx, project_id).await?;
    let expense_ids: Vec<i32> = expenses.iter().map(|e| e.id).collect();

    if expense_ids.is_empty() {
        return Ok(vec![]);
    }

    let payments =
        payments_repository::get_payments_by_expense_ids(&mut *tx, expense_ids).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(payments)
}

#[get("/api/v1/projects/{project_id}/expenses/summary")]
pub async fn get_summary_by_project_id(project_id: Uuid) -> Result<UserSummary, ServerFnError> {
    let pool = get_db().await;
    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let summary = payments_repository::get_summary_by_project_id(&mut *tx, project_id).await?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(summary)
}
