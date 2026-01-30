use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use crate::expenses::get_expenses_by_project_id;
use crate::utils::round_currency;
use shared::{Expense, Payment, UserSummary};
#[cfg(feature = "server")]
use sqlx::{Pool, Postgres};

#[get("/api/expenses/{expense_id}/payments")]
pub async fn get_payments_by_expense_id(expense_id: i32) -> Result<Vec<Payment>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE expense_id = $1",
        expense_id
    )
    .fetch_all(&pool)
    .await
    .context("Failed get payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(payments)
}

#[server()]
pub async fn get_payments_by_user_id(user_id: i32) -> Result<Vec<Payment>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await
    .context("Failed get payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(payments)
}

// TO DELETE : Should require projectId as input, not a user list
// @deprecated
#[get("/api/expenses/summary")]
pub async fn get_summary_by_user_ids(
    user_ids: Vec<i32>,
) -> Result<HashMap<i32, f64>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE user_id = ANY($1)",
        &user_ids[..] // a bug of the parameter typechecking code requires all array parameters to be slices
    )
    .fetch_all(&pool)
    .await
    .context("Failed get payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut result: HashMap<i32, f64> = HashMap::new();

    payments.iter().for_each(|payment| {
        if let Some(existing_payment) = result.get_mut(&payment.user_id) {
            match payment.is_debt {
                true => *existing_payment = round_currency(*existing_payment - payment.amount),
                false => *existing_payment = round_currency(*existing_payment + payment.amount),
            }
        } else {
            result.insert(
                payment.user_id,
                round_currency(if payment.is_debt { -payment.amount } else { payment.amount }),
            );
        }
    });

    Ok(result)
}

// Should require projectId as input, not a user list
#[get("/api/projects/{project_id}/expenses/summary")]
pub async fn get_summary_by_project_id(project_id: Uuid) -> Result<UserSummary, ServerFnError> {
    use shared::UserBalance;

    use crate::payments::balances::get_reimbursement_suggestions;

    let pool: Pool<Postgres> = get_db().await;

    let expenses: Vec<Expense> = get_expenses_by_project_id(project_id)
        .await
        .context("Failed get expenses")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let expense_ids: Vec<i32> = expenses.into_iter().map(|expense| expense.id).collect();

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE expense_id = ANY($1)",
        &expense_ids[..] // a bug of the parameter typechecking code requires all array parameters to be slices
    )
    .fetch_all(&pool)
    .await
    .context("Failed get payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut balances: HashMap<i32, f64> = HashMap::new();

    payments.iter().for_each(|payment| {
        if let Some(existing_payment) = balances.get_mut(&payment.user_id) {
            match payment.is_debt {
                true => *existing_payment = round_currency(*existing_payment - payment.amount),
                false => *existing_payment = round_currency(*existing_payment + payment.amount),
            }
        } else {
            balances.insert(
                payment.user_id,
                round_currency(if payment.is_debt { -payment.amount } else { payment.amount }),
            );
        }
    });

    let reimbursement_suggestions = get_reimbursement_suggestions(
        balances
            .iter()
            .map(|(user_id, amount)| UserBalance { amount: *amount, user_id: *user_id })
            .collect(),
    );

    let user_summary = UserSummary { reimbursement_suggestions, summary: balances };

    Ok(user_summary)
}
