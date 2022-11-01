
use diesel::{data_types::PgTimestamp};
use diesel::Queryable;
use super::schema::{users, expenses};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
	pub id: i32,
	pub name: String,
	pub balance: Option<f64>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Expense {
	pub id: i32,
	pub author_id: i32,
	pub project_id: i32,
	pub date: NaiveDate,
	pub amount: f64,
	pub description: String,
	pub name: String,
	pub expense_type: ExpenseType
}

#[derive(Queryable)]
pub struct Project {
	pub id: i32,
	pub name: String,
	pub created_at: PgTimestamp,
	pub total_expenses: f64,
	pub currency: String
}

pub struct UserAmount {
	pub user: i32,
	pub amount: f64
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
	pub date: NaiveDate,
	pub description: Option<String>,
	pub expense_type: ExpenseType,

	pub payers: Vec<UserAmount>,
	pub debtors: Vec<UserAmount>,
	pub author_id: i32,
	pub project_id: i32,
}

#[derive(Deserialize,  Debug)]
pub struct PatchableUser {
	pub user_id: i32,
	pub name: String
}

pub enum ExpenseType {
	Expense,
	Transfer,
	Gain
}