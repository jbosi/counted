use diesel::Queryable;
use crate::schema::{payments};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};
use diesel::prelude::*;


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