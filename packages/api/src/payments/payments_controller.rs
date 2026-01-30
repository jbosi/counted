use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::payments::payments_repository;
use crate::utils::round_currency;
#[cfg(feature = "server")]
use crate::{
    expenses::expenses_repository::get_expenses_by_project_id,
    payments::balances::get_reimbursement_suggestions,
};
use shared::UserBalance;
use shared::{Expense, Payment, UserSummary};

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
    let expenses: Vec<Expense> = get_expenses_by_project_id(project_id).await?;

    let expense_ids: Vec<i32> = expenses.into_iter().map(|expense| expense.id).collect();

    let payments = payments_repository::get_payments_by_expense_ids(expense_ids).await?;

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
