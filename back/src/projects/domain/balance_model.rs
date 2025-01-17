use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
	pub balances: Vec<UserBalance>,
	pub currency: String,
	pub total_expenses: f64,
	pub reimbursement_suggestions: Vec<ReimbursementSuggestion>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalance {
	pub amount: f64,
	pub user_id: i32,
	pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalanceComputation {
	pub amount: f64,
	pub remaining_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReimbursementSuggestion {
	pub amount: f64,
	pub user_id_debtor: i32,
	// pub user_name_debtor: String,
	pub user_id_payer: i32,
	// pub user_name_payer: String,
}