pub mod api;
pub mod sse;
pub mod view_models;

#[cfg(feature = "server")]
use sqlx::FromRow;
use std::{collections::HashMap, fmt};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct ProjectDto {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub currency: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatableProject {
    pub name: String,
    pub description: Option<String>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EditableProject {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub currency: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchProject {
    pub ids: Vec<Uuid>,
}

// -------- USER ---------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub balance: Option<f64>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)] // Accept either a single object or an array
pub enum CreatableUserBatch {
    Single(CreatableUser),
    Multiple(Vec<CreatableUser>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatableUser {
    pub name: String,
    pub project_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct UserProjects {
    pub project_id: Uuid,
    pub user_id: i32,
}

// -------- EXPENSE ---------

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Expense {
    pub id: i32,
    pub author_id: i32,
    pub project_id: Uuid,
    pub created_at: NaiveDateTime,
    pub amount: f64,
    pub description: Option<String>,
    pub name: String,
    pub expense_type: ExpenseType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewExpense {
    pub name: String,
    pub amount: f64,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub expense_type: ExpenseType,

    pub author_id: i32,
    pub project_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatableExpense {
    pub name: String,
    pub amount: f64,
    pub expense_type: ExpenseType,
    pub project_id: Uuid,
    pub payers: Vec<UserAmount>,
    pub debtors: Vec<UserAmount>,
    pub author_id: i32,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditableExpense {
    pub id: i32,
    pub name: String,
    pub amount: f64,
    pub expense_type: ExpenseType,
    pub project_id: Uuid,
    pub payers: Vec<UserAmount>,
    pub debtors: Vec<UserAmount>,
    pub author_id: i32,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAmount {
    pub user_id: i32,
    pub amount: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(sqlx::Type),
    sqlx(type_name = "expense_type", rename_all = "lowercase")
)]
pub enum ExpenseType {
    Expense,
    Transfer,
    Gain,
}

impl fmt::Display for ExpenseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// -------- PAYMENT ---------

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Payment {
    pub id: i32,
    pub expense_id: i32,
    pub user_id: i32,
    pub is_debt: bool,
    pub amount: f64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]

pub struct PaymentViewModel {
    pub id: i32,
    pub expense_id: i32,
    pub user: User,
    pub is_debt: bool,
    pub amount: f64,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewPayment {
    pub expense_id: i32,
    pub user_id: i32,
    pub is_debt: bool,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserSummary {
    pub reimbursement_suggestions: Vec<ReimbursementSuggestion>,
    pub summary: HashMap<i32, f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserBalance {
    pub amount: f64,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalanceComputation {
    pub amount: f64,
    pub remaining_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReimbursementSuggestion {
    pub amount: f64,
    pub user_id_debtor: i32,
    pub user_id_payer: i32,
}
