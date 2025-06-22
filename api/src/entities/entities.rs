use chrono::{DateTime, Utc, NaiveDateTime};
// use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::{Group, User};
use uuid::Uuid;
#[cfg(feature = "sqlx")]
use sqlx::FromRow;

#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, FromRow)]
pub struct Project {
	pub id: Uuid,
	pub name: String,
	// pub users: Vec<i32>, // TODO user_ids
	pub created_at: NaiveDateTime,
	pub currency: String,
	// pub total_expenses: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectDto {
	pub id: Uuid,
	pub name: String,
	pub users: Vec<i32>, // TODO user_ids
	pub created_at: NaiveDateTime,
	pub currency: String,
	// pub total_expenses: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewProject {
	pub name: String,
	// pub users: Vec<i32>,
	// pub total_expenses: f64,
	pub currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatableProject {
	pub name: String,
	pub users: Vec<i32>,
}

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct DbGroup {
//     pub id: i32,
//     pub name: String,
//     pub share_token: Uuid,
//     pub created_at: DateTime<Utc>,
// }

// impl From<DbGroup> for Group {
//     fn from(db_group: DbGroup) -> Self {
//         Group {
//             id: db_group.id,
//             name: db_group.name,
//             share_token: db_group.share_token,
//             created_at: db_group.created_at
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct DbUser {
//     pub id: i32,
//     pub group_id: i32,
//     pub name: String,
// }

// impl From<DbUser> for User {
//     fn from(db_user: DbUser) -> Self {
//         User {
//             id: db_user.id,
//             group_id: db_user.group_id,
//             name: db_user.name,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct DbExpense {
//     pub id: i32,
//     pub group_id: i32,
//     pub description: String,
//     pub amount: Decimal,
//     pub paid_by_user_id: i32,
//     pub date: DateTime<Utc>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
// pub struct DbUserBalance {
//     pub user_id: i32,
//     pub user_name: String,
//     pub total_paid: Decimal,
//     pub total_share: Decimal,
//     pub balance: Decimal,
// }

// // Payloads pour les requêtes de création
// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct DbCreateExpensePayload {
//     pub description: String,
//     pub amount: Decimal,
//     pub paid_by_user_id: i32,
//     pub participant_ids: Vec<i32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct DbCreateUserPayload {
//     pub name: String,
// }

// Nouvelle structure pour les données complètes d'un groupe,
// afin d'avoir un type clair pour la réponse de l'API.
// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct DbFullGroupDetails {
//     pub group: Group,
//     pub users: Vec<User>,
//     // Simplifié pour le frontend
//     pub expenses: Vec<ExpenseSummary>,
//     pub balances: Vec<UserBalance>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
// pub struct DbExpenseSummary {
//     pub id: i32,
//     pub description: String,
//     pub amount: Decimal,
//     pub paid_by_user_id: i32,
//     pub paid_by_user_name: String,
// }
