use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub share_token: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: i32,
    pub group_id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: i32,
    pub group_id: i32,
    pub description: String,
    pub amount: Decimal,
    pub paid_by_user_id: i32,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserBalance {
    pub user_id: i32,
    pub user_name: String,
    pub total_paid: Decimal,
    pub total_share: Decimal,
    pub balance: Decimal,
}

// Payloads pour les requêtes de création
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExpensePayload {
    pub description: String,
    pub amount: Decimal,
    pub paid_by_user_id: i32,
    pub participant_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub name: String,
}

// Nouvelle structure pour les données complètes d'un groupe,
// afin d'avoir un type clair pour la réponse de l'API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullGroupDetails {
    pub group: Group,
    pub users: Vec<User>,
    // Simplifié pour le frontend
    pub expenses: Vec<ExpenseSummary>,
    pub balances: Vec<UserBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpenseSummary {
    pub id: i32,
    pub description: String,
    pub amount: Decimal,
    pub paid_by_user_id: i32,
    pub paid_by_user_name: String,
}
