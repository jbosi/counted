use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ExpenseQueryParams {
    pub project_id: Option<Uuid>,
}