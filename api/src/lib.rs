//! This crate contains all shared fullstack server functions.
mod db;

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
use sqlx::{FromRow, PgPool, Pool, QueryBuilder, Postgres};
#[cfg(feature = "server")]
use crate::db::get_db;
use shared::{Project, User, CreatableUser, CreatableProject, CreatableExpense, UserAmount, NewPayment, ExpenseType, Expense, Payment};


// --- PROJECTS ---

#[server()]
pub async fn get_project(project_id: Uuid) -> Result<Project, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: Project = sqlx::query_as("SELECT * FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_one(&pool)
        .await?;

    Ok(projects)
}

#[server()]
pub async fn add_project(project: CreatableProject) -> Result<Uuid, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let project_id: Uuid = sqlx::query_scalar!("INSERT INTO projects(name, description, currency) VALUES ($1, $2, $3) RETURNING id", project.name, project.description, "EUR")
        .fetch_one(&pool)
        .await?;

    Ok(project_id)
}

#[server()]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let projects: Vec<Project> = sqlx::query_as("SELECT id, name, created_at, currency, description FROM projects")
        .fetch_all(&pool)
        .await?;

    Ok(projects)
}

// --- USERS ---

#[server()]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    // TODO Passer directement la struct en 1er argument
    let users: Vec<User> = sqlx::query_as("SELECT id, name, balance, created_at FROM users")
        .fetch_all(&pool)
        .await?;

    Ok(users)
}

#[server()]
pub async fn add_user(user: CreatableUser) -> Result<i32, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user_id: i32 = sqlx::query_scalar!("INSERT INTO users(name) VALUES ($1) RETURNING id", user.name)
        .fetch_one(&pool)
        .await?;

    sqlx::query!("INSERT INTO user_projects(user_id, project_id) VALUES ($1, $2)", user_id, user.project_id)
        .execute(&pool)
        .await?;

    Ok(user_id)
}

#[server()]
pub async fn get_users_by_project_id(project_id: Uuid) -> Result<Vec<User>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let user_ids: Vec<i32> = sqlx::query!("SELECT user_id FROM user_projects WHERE project_id = $1", project_id)
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|row| row.user_id)
        .collect();

    if user_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("SELECT id, name, balance, created_at FROM users WHERE id IN (");

    let mut separated = query_builder.separated(", ");
    for id in user_ids {
        separated.push_bind(id);
    }
    separated.push_unseparated(")");

    let query = query_builder.build_query_as::<User>();
    let users: Vec<User> = query
        .fetch_all(&pool)
        .await?;

    // TODO
    // const QUERY: &str = "SELECT u.id, u.name, u.balance, u.created_at
    //     FROM users u
    //     JOIN user_projects up ON up.user_id = u.id
    //     JOIN projects p ON p.id = up.project_id
    //     WHERE project_id = '553b5fc6-3e91-4c85-af6f-5d7a2e6bf9ff'";
    //
    // let users: Vec<User> = sqlx::query(QUERY)
    //     .fetch_all(&pool)
    //     .await?;

    Ok(users)
}

// --- EXPENSES ---

#[server()]
pub async fn add_expense(expense: CreatableExpense) -> Result<i32, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let created_expense_id: i32 = sqlx::query!(r"
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
        .await?
        .id;

    let payers = Some(expense.clone().payers);
    let debtors = Some(expense.clone().debtors);

    let creatable_payments: Vec<NewPayment> = forge_creatable_payments_from_expense(payers, debtors, created_expense_id);

    let expense_ids: Vec<i32> = creatable_payments.iter().map(|p| p.expense_id).collect();
    let user_ids: Vec<i32> = creatable_payments.iter().map(|p| p.user_id).collect();
    let is_debts: Vec<bool> = creatable_payments.iter().map(|p| p.is_debt).collect();
    let amounts: Vec<f64> = creatable_payments.iter().map(|p| p.amount).collect();

    let sql = r"
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
        ) RETURNING id";

    sqlx::query!(r"
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
    .await?;

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
        .await?;

    Ok(expenses)
}

fn forge_creatable_payments_from_expense(payers: Option<Vec<UserAmount>>, debtors: Option<Vec<UserAmount>>, created_expense_id: i32) -> Vec<NewPayment> {
    let mut debtors_result: Vec<UserAmount> = vec![];
    if let Some(debtors_unwrapped) = debtors {
        debtors_result = debtors_unwrapped;
    }

    let mut payers_result: Vec<UserAmount> = vec![];
    if let Some(payers_unwrapped) = payers {
        payers_result = payers_unwrapped;
    }

    let creatable_debtors: Vec<NewPayment> = debtors_result.into_iter()
        .filter(|d| d.amount != 0.0)
        .map(|d| NewPayment {
            amount: d.amount,
            expense_id: created_expense_id,
            user_id: d.user_id,
            is_debt: true
        }).collect();

    let creatable_payers: Vec<NewPayment> = payers_result.into_iter()
        .filter(|d| d.amount != 0.0)
        .map(|p| NewPayment {
            amount: p.amount,
            expense_id: created_expense_id,
            user_id: p.user_id,
            is_debt: false
        }).collect();

    [creatable_debtors, creatable_payers].concat()
}

// --- PAYMENTS ---

#[server()]
pub async fn get_payments_by_expense_id(expense_id: i32) -> Result<Vec<Payment>, ServerFnError> {
    let pool: Pool<Postgres> = get_db().await;

    let payments: Vec<Payment> = sqlx::query_as!(
        Payment,
        "SELECT id, expense_id, user_id, is_debt, amount, created_at \
        FROM payments \
        WHERE expense_id = $1",
        expense_id)
        .fetch_all(&pool)
        .await?;

    Ok(payments)
}