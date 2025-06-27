#[cfg(feature = "server")]
use sqlx::FromRow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    // pub users: Vec<i32>, // TODO user_ids
    pub created_at: NaiveDateTime,
    pub currency: String,
    // pub total_expenses: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDto {
    pub id: Uuid,
    pub name: String,
    pub users: Vec<i32>, // TODO user_ids
    pub created_at: NaiveDateTime,
    pub currency: String,
    // pub total_expenses: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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