use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProjectQueryParams {
	pub user_id: Option<i32>,
}