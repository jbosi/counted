use diesel::Queryable;
use crate::schema::{users};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug, Identifiable, Selectable, PartialEq)]
pub struct User {
	pub id: i32,
	pub name: String,
	pub balance: Option<f64>,
	pub created_at: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatableUser {
	pub name: String
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
	pub name: String
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