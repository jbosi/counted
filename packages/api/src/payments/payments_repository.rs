use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::db::get_db;
#[cfg(feature = "server")]
use crate::expenses::get_expenses_by_project_id;
#[cfg(feature = "server")]
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dioxus::logger::tracing::info;
use shared::{Expense, Payment};
#[cfg(feature = "server")]
use sqlx::{FromRow, PgPool, Pool, Postgres, QueryBuilder};

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
                true => *existing_payment -= payment.amount,
                false => *existing_payment += payment.amount,
            }
        } else {
            result.insert(
                payment.user_id,
                if payment.is_debt { -payment.amount } else { payment.amount },
            );
        }
    });

    Ok(result)
}

// Should require projectId as input, not a user list
#[get("/api/projects/{project_id}/expenses/summary")]
pub async fn get_summary_by_project_id(
    project_id: Uuid,
) -> Result<HashMap<i32, f64>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let expenses: Vec<Expense> = get_expenses_by_project_id(project_id)
        .await
        .context("Failed get expenses")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE expense_id = ANY($1)",
        &expenses[..] // a bug of the parameter typechecking code requires all array parameters to be slices
    )
    .fetch_all(&pool)
    .await
    .context("Failed get payments")
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut result: HashMap<i32, f64> = HashMap::new();

    payments.iter().for_each(|payment| {
        if let Some(existing_payment) = result.get_mut(&payment.user_id) {
            match payment.is_debt {
                true => *existing_payment -= payment.amount,
                false => *existing_payment += payment.amount,
            }
        } else {
            result.insert(
                payment.user_id,
                if payment.is_debt { -payment.amount } else { payment.amount },
            );
        }
    });

    Ok(result)
}
