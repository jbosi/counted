use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UsersQueryParams {
	pub project_id: Option<Uuid>,
}