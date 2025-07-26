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

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::db::get_db;
use shared::{
    CreatableExpense, CreatableProject, CreatableUser, Expense, ExpenseType, NewPayment, Payment,
    Project, User, UserAmount,
};
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
