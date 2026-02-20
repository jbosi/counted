use dioxus::fullstack::Json;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use shared::{ProjectDto, User};

#[cfg(feature = "server")]
use std::collections::HashMap;

#[cfg(feature = "server")]
use shared::{
    CreatableExpense, CreatableProject, CreatableUser, ExpenseType, NewPayment, UserAmount,
};

#[cfg(feature = "server")]
use super::tricount_client;
#[cfg(feature = "server")]
use crate::expenses::expenses_repository;
#[cfg(feature = "server")]
use crate::payments::payments_repository;
#[cfg(feature = "server")]
use crate::projects::projects_repository;
#[cfg(feature = "server")]
use crate::users::users_repository;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TricountImportRequest {
    pub tricount_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TricountImportResponse {
    pub project: ProjectDto,
    pub users: Vec<User>,
    pub expenses_count: usize,
}

#[post("/api/v1/import/tricount")]
pub async fn import_tricount(
    Json(payload): Json<TricountImportRequest>,
) -> Result<TricountImportResponse, ServerFnError> {
    let key = tricount_client::extract_tricount_key(&payload.tricount_key);

    // Fetch from Tricount API
    let registry = tricount_client::fetch_tricount(&key).await?;

    // Create project
    let project_id = projects_repository::add_project(
        CreatableProject {
            name: registry.title.clone(),
            description: Some("Imported from Tricount".to_string()),
            currency: Some(registry.currency.clone()),
        },
        None, // Tricount imports are anonymous (no owner account)
    )
    .await?;

    // Extract members and create users
    let members: Vec<_> = registry.memberships.iter().filter_map(|m| m.non_user.as_ref()).collect();

    let creatable_users: Vec<CreatableUser> = members
        .iter()
        .map(|m| CreatableUser { name: m.alias.display_name.clone(), project_id })
        .collect();

    let created_users = users_repository::add_users(creatable_users).await?;

    // Build UUID -> user_id mapping
    let uuid_to_user_id: HashMap<String, i32> = members
        .iter()
        .zip(created_users.iter())
        .map(|(membership, user)| (membership.uuid.clone(), user.id))
        .collect();

    // Create expenses and payments
    let mut expenses_count = 0;

    for entry_wrapper in &registry.all_registry_entry {
        let entry = match &entry_wrapper.entry {
            Some(e) => e,
            None => continue,
        };

        // --- Amount ---
        // Tricount stores amounts as negative values (debtor perspective) — take abs
        let amount: f64 = entry["amount"]["value"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0)
            .abs();
        if amount == 0.0 {
            continue;
        }

        // --- Payer ---
        let payer_uuid = entry["membership_owned"]["RegistryMembershipNonUser"]["uuid"].as_str();
        let payer_id = match payer_uuid.and_then(|u| uuid_to_user_id.get(u)) {
            Some(id) => *id,
            None => continue,
        };

        // --- Debtors from allocations ---
        let debtors: Vec<UserAmount> = entry["allocations"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|alloc| {
                let uuid = alloc["membership"]["RegistryMembershipNonUser"]["uuid"].as_str()?;
                let uid = uuid_to_user_id.get(uuid)?;
                let amt: f64 = alloc["amount"]["value"].as_str()?.parse().ok()?;
                Some(UserAmount { user_id: *uid, amount: amt.abs() })
            })
            .collect();

        // --- Date ---
        // "date" is the user-visible field; fall back to "created".
        // Format varies — slice first 10 chars ("YYYY-MM-DD") to handle any suffix.
        let date = ["date", "object_date", "updated", "created"]
            .iter()
            .find_map(|field| entry[field].as_str().and_then(parse_tricount_date))
            .unwrap_or_else(|| chrono::Local::now().date_naive());

        // --- Expense type ---
        // type_transaction="BALANCE" means a reimbursement/transfer between members
        let expense_type = match entry["type_transaction"].as_str() {
            Some("BALANCE") => ExpenseType::Transfer,
            _ => ExpenseType::Expense,
        };

        let creatable_expense = CreatableExpense {
            name: entry["description"].as_str().unwrap_or("").to_string(),
            amount,
            expense_type,
            project_id,
            payers: vec![UserAmount { user_id: payer_id, amount }],
            debtors,
            author_id: payer_id,
            description: None,
            date,
        };

        // Create expense in DB
        let expense_id = expenses_repository::add_expense(creatable_expense.clone()).await?;

        // Create payments (mirrors expenses_controller::add_expense logic)
        let mut payments: Vec<NewPayment> = Vec::new();

        for debtor in &creatable_expense.debtors {
            if debtor.amount != 0.0 {
                payments.push(NewPayment {
                    expense_id,
                    user_id: debtor.user_id,
                    is_debt: true,
                    amount: debtor.amount,
                });
            }
        }
        for payer in &creatable_expense.payers {
            if payer.amount != 0.0 {
                payments.push(NewPayment {
                    expense_id,
                    user_id: payer.user_id,
                    is_debt: false,
                    amount: payer.amount,
                });
            }
        }

        payments_repository::add_payments(payments).await?;
        expenses_count += 1;
    }

    let project = projects_repository::get_project(project_id).await?;

    Ok(TricountImportResponse { project, users: created_users, expenses_count })
}

fn parse_tricount_date(s: &str) -> Option<chrono::NaiveDate> {
    // All bunq/Tricount date strings start with "YYYY-MM-DD".
    // Slicing the first 10 chars handles every suffix variant:
    //   "2024-01-15", "2024-01-15 00:00:00", "2024-01-15 00:00:00.000000",
    //   "2024-01-15T00:00:00+00:00", etc.
    let prefix = s.get(..10)?;
    chrono::NaiveDate::parse_from_str(prefix, "%Y-%m-%d").ok()
}
