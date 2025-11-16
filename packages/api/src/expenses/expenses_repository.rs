use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature = "server")]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use crate::sse::BROADCASTER;
use shared::sse::EventSSE;
use shared::{CreatableExpense, Expense, ExpenseType, NewPayment, Payment, UserAmount};
#[cfg(feature = "server")]
use sqlx::{FromRow, PgPool, Pool, Postgres, QueryBuilder};

#[server()]
pub async fn add_expense(expense: CreatableExpense) -> Result<i32, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let created_expense_id: i32 = sqlx::query!(
        r"
            INSERT INTO expenses
            (
                name,
                amount,
                expense_type,
                project_id,
                author_id,
                description
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            ) RETURNING id",
        expense.clone().name,
        expense.clone().amount,
        expense.clone().expense_type as ExpenseType,
        expense.clone().project_id,
        expense.clone().author_id,
        expense.clone().description
    )
    .fetch_one(&pool)
    .await
    .context("Failed to create expense")
    .map_err(|e| ServerFnError::new(e.to_string()))?
    .id;

    let payers = Some(expense.clone().payers);
    let debtors = Some(expense.clone().debtors);

    let creatable_payments: Vec<NewPayment> =
        forge_creatable_payments_from_expense(payers, debtors, created_expense_id);

    let expense_ids: Vec<i32> = creatable_payments.iter().map(|p| p.expense_id).collect();
    let user_ids: Vec<i32> = creatable_payments.iter().map(|p| p.user_id).collect();
    let is_debts: Vec<bool> = creatable_payments.iter().map(|p| p.is_debt).collect();
    let amounts: Vec<f64> = creatable_payments.iter().map(|p| p.amount).collect();

    sqlx::query!(
        r"
        INSERT INTO payments
         (
            expense_id,
            user_id,
            is_debt,
            amount
        ) SELECT * FROM UNNEST(
            $1::INT4[],
            $2::INT4[],
            $3::BOOL[],
            $4::FLOAT8[]
        ) RETURNING id",
        &expense_ids,
        &user_ids,
        &is_debts,
        &amounts
    )
    .fetch_all(&pool)
    .await
    .context("Failed add payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    BROADCASTER
        .broadcast(
            axum::response::sse::Event::default()
                .event::<String>(EventSSE::ExpenseCreated.to_string())
                .data(EventSSE::ExpenseCreated.to_string()),
        )
        .await;

    Ok(created_expense_id)
}

#[server()]
pub async fn get_expenses_by_project_id(project_id: Uuid) -> Result<Vec<Expense>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;
    let expenses: Vec<Expense> = sqlx::query_as!(
        Expense,
        "SELECT id, author_id, project_id, created_at, amount, description, name, expense_type as \"expense_type: ExpenseType\" \
        FROM expenses \
        WHERE project_id = $1",
        project_id)
        .fetch_all(&pool)
        .await
        .context("Failed to get expenses")
        .map_err(|e| ServerFnError::new(e.to_string()))?;;

    Ok(expenses)
}

#[server()]
pub async fn get_expense_by_id(expense_id: i32) -> Result<Expense, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;
    let expense: Expense = sqlx::query_as!(
        Expense,
        "SELECT id, author_id, project_id, created_at, amount, description, name, expense_type as \"expense_type: ExpenseType\" FROM expenses WHERE id = $1", expense_id)
        .fetch_one(&pool)
        .await
        .context("Failed to get expense")
        .map_err(|e| ServerFnError::new(e.to_string()))?;;

    Ok(expense)
}

#[server()]
pub async fn delete_expense_by_id(expense_id: i32) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    sqlx::query!("DELETE FROM payments WHERE expense_id = $1", expense_id).execute(&pool).await
        .context("Failed to delete payment")
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    sqlx::query!("DELETE FROM expenses WHERE id = $1", expense_id).execute(&pool).await
        .context("Failed to delete expense")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    BROADCASTER
        .broadcast(
            axum::response::sse::Event::default()
                .event::<String>(EventSSE::ExpenseDeleted.to_string())
                .data(EventSSE::ExpenseDeleted.to_string()),
        )
        .await;

    Ok(())
}

fn forge_creatable_payments_from_expense(
    payers: Option<Vec<UserAmount>>,
    debtors: Option<Vec<UserAmount>>,
    created_expense_id: i32,
) -> Vec<NewPayment> {
    let mut debtors_result: Vec<UserAmount> = vec![];
    if let Some(debtors_unwrapped) = debtors {
        debtors_result = debtors_unwrapped;
    }

    let mut payers_result: Vec<UserAmount> = vec![];
    if let Some(payers_unwrapped) = payers {
        payers_result = payers_unwrapped;
    }

    let creatable_debtors: Vec<NewPayment> = debtors_result
        .into_iter()
        .filter(|d| d.amount != 0.0)
        .map(|d| NewPayment {
            amount: d.amount,
            expense_id: created_expense_id,
            user_id: d.user_id,
            is_debt: true,
        })
        .collect();

    let creatable_payers: Vec<NewPayment> = payers_result
        .into_iter()
        .filter(|d| d.amount != 0.0)
        .map(|p| NewPayment {
            amount: p.amount,
            expense_id: created_expense_id,
            user_id: p.user_id,
            is_debt: false,
        })
        .collect();

    [creatable_debtors, creatable_payers].concat()
}
