use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use shared::{CreatableExpense, EditableExpense, Expense, ExpenseType};
#[cfg(feature = "server")]
use sqlx::{Pool, Postgres};

#[cfg(feature = "server")]
pub async fn add_expense(expense: CreatableExpense) -> Result<i32, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let created_expense_id: i32 = sqlx::query_scalar!(
        r"
            INSERT INTO expenses
            (
                name,
                amount,
                expense_type,
                project_id,
                author_id,
                description,
                date
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7
            ) RETURNING id",
        expense.clone().name,
        expense.clone().amount,
        expense.clone().expense_type as ExpenseType,
        expense.clone().project_id,
        expense.clone().author_id,
        expense.clone().description,
        expense.clone().date
    )
    .fetch_one(&pool)
    .await
    .context("Failed to create expense")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(created_expense_id)
}

#[cfg(feature = "server")]
pub async fn edit_expense(expense: EditableExpense) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    sqlx::query!(
        r#"
        UPDATE expenses
        SET
            name          = $1,
            amount        = $2,
            expense_type  = $3,
            project_id    = $4,
            author_id     = $5,
            description   = $6,
            date          = $7
        WHERE id = $8
        "#,
        expense.name,
        expense.amount,
        expense.clone().expense_type as ExpenseType,
        expense.project_id,
        expense.author_id,
        expense.description,
        expense.date,
        expense.id
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn get_expenses_by_project_id(project_id: Uuid) -> Result<Vec<Expense>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;
    let expenses: Vec<Expense> = sqlx::query_as!(
        Expense,
        "SELECT id, author_id, project_id, created_at, date, amount, description, name, expense_type as \"expense_type: ExpenseType\" \
        FROM expenses \
        WHERE project_id = $1",
        project_id)
        .fetch_all(&pool)
        .await
        .context("Failed to get expenses")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(expenses)
}

#[cfg(feature = "server")]
pub async fn get_expense_by_id(expense_id: i32) -> Result<Expense, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;
    let expense: Expense = sqlx::query_as!(
        Expense,
        "SELECT id, author_id, project_id, created_at, date, amount, description, name, expense_type as \"expense_type: ExpenseType\" FROM expenses WHERE id = $1", expense_id)
        .fetch_one(&pool)
        .await
        .context("Failed to get expense")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(expense)
}

#[cfg(feature = "server")]
pub async fn delete_expense(expense_id: i32) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    sqlx::query!("DELETE FROM expenses WHERE id = $1", expense_id)
        .execute(&pool)
        .await
        .context("Failed to delete expense")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
