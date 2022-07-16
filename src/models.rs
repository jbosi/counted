
use diesel::data_types::PgTimestamp;
use diesel::Queryable;
use super::schema::users;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub balance: Option<f64>,
}

#[derive(Queryable)]
pub struct Expense {
    pub id: i32,
    pub paid_for_id: i32,
    pub paid_by_id: i32,
    pub author_id: i32,
    pub project_id: i32,
    pub date: PgTimestamp,
    pub amount: f64,
    pub description: String,
    pub name: String,
    pub expense_type: String
}

#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub created_at: PgTimestamp,
    pub total_expenses: f64,
    pub currency: String
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub balance: Option<f64>,
}