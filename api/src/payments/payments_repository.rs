use dioxus::prelude::*;
use std::collections::HashMap;

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
use shared::Payment;
#[cfg(feature = "server")]
use sqlx::{FromRow, PgPool, Pool, Postgres, QueryBuilder};

#[server()]
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
    .await?;

    Ok(payments)
}

#[server()]
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
    .await?;

    let mut result: HashMap<i32, f64> = HashMap::new();

    payments.iter().for_each(|payment| {
        if let Some(existing_payment) = result.get_mut(&payment.user_id) {
            *existing_payment += payment.amount;
        } else {
            result.insert(payment.user_id, payment.amount);
        }
    });

    Ok(result)
}
