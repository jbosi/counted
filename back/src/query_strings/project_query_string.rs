use actix_web::{web, App};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProjectQueryParams {
	pub user_id: Option<i32>,
}