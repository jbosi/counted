use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserProjectsQueryParams {
	pub user_id: Option<i32>,
	pub project_id: Option<Uuid>,
}