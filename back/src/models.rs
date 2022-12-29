use diesel::Queryable;
use super::schema::{users, expenses, payments};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime};
use diesel_derive_enum::DbEnum;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
	pub id: i32,
	pub name: String,
	pub balance: Option<f64>,
	pub created_at: Option<NaiveDateTime>
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Expense {
	pub id: i32,
	pub author_id: i32,
	pub project_id: i32,
	pub date: NaiveDate,
	pub amount: f64,
	pub description: Option<String>,
	pub name: String,
	pub expense_type: ExpenseType,
}

#[derive(Queryable)]
pub struct Project {
	pub id: i32,
	pub name: String,
	pub created_at: NaiveDate,
	pub total_expenses: f64,
	pub currency: String
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name="users"]
pub struct NewUser {
	pub name: String,
	pub balance: Option<f64>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name="expenses"]
pub struct NewExpense {
	pub name: String,
	pub amount: f64,
	pub description: Option<String>,
	pub expense_type: ExpenseType,

	pub author_id: i32,
	pub project_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatableExpense {
	pub name: String,
	pub amount: f64,
	pub description: Option<String>,
	pub expense_type: ExpenseType,

	pub payers: Vec<UserAmount>,
	pub debtors: Vec<UserAmount>,
	pub author_id: i32,
	pub project_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAmount {
	pub user_id: i32,
	pub amount: f64
}

#[derive(Deserialize, Debug)]
pub struct PatchableUser {
	pub user_id: i32,
	pub name: String
}

#[derive(Debug, PartialEq, DbEnum, Clone, Serialize, Deserialize)]
pub enum ExpenseType {
	Expense,
	Transfer,
	Gain
}

#[derive(diesel::sql_types::SqlType, diesel::query_builder::QueryId)]
#[diesel(postgres_type(name = "expense_type"))]
pub struct ExpenseTypeMapping;

#[derive(Insertable, Serialize, Deserialize, Debug)]
pub struct Payment {
	id: i32,
	expense_id: i32,
	user_id: i32,
	is_debt: bool,
	amount: f64,
	created_at: NaiveDateTime
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name="payments"]
pub struct NewPayment {
	pub expense_id: i32,
	pub user_id: i32,
	pub is_debt: bool,
	pub amount: f64,
}