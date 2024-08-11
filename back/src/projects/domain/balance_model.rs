use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
	pub balances: Vec<UserBalance>,
	pub currency: String,
	pub total_expenses: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalance {
	pub amount: f64,
	pub user_id: i32,
}
