use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
use crate::utils::round_currency;
#[cfg(feature = "server")]
use crate::{
    expenses::expenses_repository::get_expenses_by_project_id,
    payments::balances::get_reimbursement_suggestions,
};
use shared::{Expense, ExpenseType, NewPayment, Payment, UserBalance, UserSummary};

#[cfg(feature = "server")]
use sqlx::{Pool, Postgres};

#[cfg(feature = "server")]
pub async fn add_payments(creatable_payments: Vec<NewPayment>) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let expense_ids: Vec<i32> = creatable_payments.iter().map(|p| p.expense_id).collect();
    let user_ids: Vec<i32> = creatable_payments.iter().map(|p| p.user_id).collect();
    let is_debts: Vec<bool> = creatable_payments.iter().map(|p| p.is_debt).collect();
    let amounts: Vec<f64> = creatable_payments.iter().map(|p| p.amount).collect();

    sqlx::query_scalar::<_, i32>(
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
    )
    .bind(&expense_ids)
    .bind(&user_ids)
    .bind(&is_debts)
    .bind(&amounts)
    .fetch_all(&pool)
    .await
    .context("Failed add payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
pub async fn get_payments_by_expense_ids(
    expense_ids: Vec<i32>,
) -> Result<Vec<Payment>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

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

    Ok(payments)
}

#[cfg(feature = "server")]
pub async fn delete_payments_by_expense_id(expense_id: i32) -> Result<(), ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    sqlx::query!("DELETE FROM payments WHERE expense_id = $1", expense_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn get_summary_by_project_id(project_id: Uuid) -> Result<UserSummary, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let expenses: Vec<Expense> = get_expenses_by_project_id(project_id)
        .await
        .context("Failed get expenses")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let expense_ids: Vec<i32> = expenses.iter().map(|expense| expense.id).collect();

    // Create a lookup map from expense_id to expense_type for balance calculation
    let expense_type_by_id: HashMap<i32, ExpenseType> =
        expenses.into_iter().map(|expense| (expense.id, expense.expense_type)).collect();

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
        let expense_type = expense_type_by_id
            .get(&payment.expense_id)
            .expect("Payment references non-existent expense");

        // For Gain expenses, invert the balance logic
        let multiplier = if matches!(expense_type, ExpenseType::Gain) { -1.0 } else { 1.0 };
        let base_adjustment = if payment.is_debt { -payment.amount } else { payment.amount };
        let balance_adjustment = base_adjustment * multiplier;

        if let Some(existing_balance) = balances.get_mut(&payment.user_id) {
            *existing_balance = round_currency(*existing_balance + balance_adjustment);
        } else {
            balances.insert(payment.user_id, round_currency(balance_adjustment));
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
