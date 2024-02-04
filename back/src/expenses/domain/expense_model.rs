use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::Queryable;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::expenses;
use crate::users::domain::user_model::UserAmount;

#[derive(Queryable, Serialize, Deserialize, Debug, Selectable, Identifiable, PartialEq)]
pub struct Expense {
	pub id: i32,
	pub author_id: i32,
	pub project_id: Uuid,
	pub date: NaiveDate,
	pub amount: f64,
	pub description: Option<String>,
	pub name: String,
	pub expense_type: ExpenseType,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = expenses)]
pub struct NewExpense {
	pub name: String,
	pub amount: f64,
	pub date: NaiveDate,
	pub description: Option<String>,
	pub expense_type: ExpenseType,

	pub author_id: i32,
	pub project_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
pub struct PatchableExpense {
	pub name: Option<String>,
	pub amount: Option<f64>,
	pub description: Option<String>,
	pub expense_type: Option<ExpenseType>,

	pub payers: Option<Vec<UserAmount>>,
	pub debtors: Option<Vec<UserAmount>>,
	pub author_id: i32,
}

#[derive(Debug, PartialEq, DbEnum, Clone, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::ExpenseType"]
pub enum ExpenseType {
	Expense,
	Transfer,
	Gain
}

#[derive(diesel::sql_types::SqlType, diesel::query_builder::QueryId)]
#[diesel(postgres_type(name = "expense_type"))]
pub struct ExpenseTypeMapping;