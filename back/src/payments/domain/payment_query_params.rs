use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaymentQueryParams {
    pub user_id: Option<i32>,
}