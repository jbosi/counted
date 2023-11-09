use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ExpensesQueryParams {
    pub project_id: Option<Uuid>,
}