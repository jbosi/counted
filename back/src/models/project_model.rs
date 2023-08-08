use diesel::Queryable;
use uuid::Uuid;
use crate::schema::{projects};
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};
use diesel::prelude::*;


#[derive(Queryable, Serialize, Deserialize, Debug, Selectable, Identifiable, Copy, PartialEq)]
pub struct Project {
	pub id: Uuid,
	pub name: String,
	// pub users: Vec<i32>, // TODO user_ids
	pub created_at: NaiveDateTime,
	pub currency: String,
	// pub total_expenses: f64,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectDto {
	pub id: Uuid,
	pub name: String,
	pub users: Vec<i32>, // TODO user_ids
	pub created_at: NaiveDateTime,
	pub currency: String,
	// pub total_expenses: f64,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = projects)]
pub struct NewProject {
	pub name: String,
	// pub users: Vec<i32>,
	// pub total_expenses: f64,
	pub currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatableProject {
	pub name: String,
	pub users: Vec<i32>,
}
