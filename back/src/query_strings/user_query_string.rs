use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserQueryParams {
	pub project_id: Option<Uuid>,
}