use diesel::Queryable;
use uuid::Uuid;
use crate::schema::{payments};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, NaiveDate};
use diesel::prelude::*;

use super::expense_model::ExpenseType;


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Payment {
	pub id: i32,
	pub expense_id: i32,
	pub user_id: i32,
	pub is_debt: bool,
	pub amount: f64,
	pub created_at: NaiveDateTime
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name="payments"]
pub struct NewPayment {
	pub expense_id: i32,
	pub user_id: i32,
	pub is_debt: bool,
	pub amount: f64,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ExpensePayments {
	pub id: i32,
	pub author_id: i32,
	pub project_id: Uuid,
	pub date: NaiveDate,
	pub amount: f64,
	pub description: Option<String>,
	pub name: String,
	pub expense_type: ExpenseType,
	pub created_at: NaiveDateTime,
	pub payments: Vec<Payment>
}