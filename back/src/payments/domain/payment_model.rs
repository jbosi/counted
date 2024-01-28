use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

use crate::expenses::domain::expense_model::ExpenseType;
use crate::schema::payments;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, ToSchema, Identifiable, Selectable)]
pub struct Payment {
	pub id: i32,
	pub expense_id: i32,
	pub user_id: i32,
	pub is_debt: bool,
	pub amount: f64,
	pub created_at: NaiveDateTime
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = payments)]
pub struct NewPayment {
	pub expense_id: i32,
	pub user_id: i32,
	pub is_debt: bool,
	pub amount: f64,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct ExpenseDto {
	pub id: i32,
	pub author_id: i32,
	pub project_id: Uuid,
	pub date: NaiveDate,
	pub amount: f64,
	pub description: Option<String>,
	pub name: String,
	pub expense_type: ExpenseType,
	// pub created_at: NaiveDateTime,
	pub payments: Vec<Payment>
}