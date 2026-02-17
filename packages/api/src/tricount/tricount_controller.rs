use dioxus::fullstack::Json;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use shared::{ProjectDto, User};

#[cfg(feature = "server")]
use std::collections::HashMap;

#[cfg(feature = "server")]
use shared::{CreatableExpense, CreatableProject, CreatableUser, ExpenseType, NewPayment, UserAmount};

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

#[post("/api/import/tricount")]
pub async fn import_tricount(
    Json(payload): Json<TricountImportRequest>,
) -> Result<TricountImportResponse, ServerFnError> {
    let key = tricount_client::extract_tricount_key(&payload.tricount_key);

    // 1. Fetch from Tricount API
    let registry = tricount_client::fetch_tricount(&key).await?;

    // 2. Create project
    let project_id = projects_repository::add_project(CreatableProject {
        name: registry.title.clone(),
        description: Some("Imported from Tricount".to_string()),
        currency: Some(registry.currency.clone()),
    })
    .await?;

    // 3. Extract members and create users
    let members: Vec<_> = registry
        .memberships
        .iter()
        .filter_map(|m| m.non_user.as_ref())
        .collect();

    let creatable_users: Vec<CreatableUser> = members
        .iter()
        .map(|m| CreatableUser {
            name: m.alias.display_name.clone(),
            project_id,
        })
        .collect();

    let created_users = users_repository::add_users(creatable_users).await?;

    // 4. Build UUID -> user_id mapping
    let uuid_to_user_id: HashMap<String, i32> = members
        .iter()
        .zip(created_users.iter())
        .map(|(membership, user)| (membership.uuid.clone(), user.id))
        .collect();

    // 5. Create expenses and payments
    let mut expenses_count = 0;

    for entry_wrapper in &registry.all_registry_entry {
        let entry = match &entry_wrapper.entry {
            Some(e) => e,
            None => continue,
        };

        let amount: f64 = entry.amount.value.parse().unwrap_or(0.0);
        if amount == 0.0 {
            continue;
        }

        // Find payer UUID from membership_owned
        let payer_uuid = entry
            .membership_owned
            .non_user
            .as_ref()
            .map(|m| &m.uuid);

        let payer_id = match payer_uuid {
            Some(uuid) => match uuid_to_user_id.get(uuid) {
                Some(id) => *id,
                None => continue,
            },
            None => continue,
        };

        // Build debtors from allocations
        let debtors: Vec<UserAmount> = entry
            .allocations
            .iter()
            .filter_map(|alloc| {
                let uuid = alloc.membership.non_user.as_ref()?.uuid.as_str();
                let uid = uuid_to_user_id.get(uuid)?;
                let amt: f64 = alloc.amount.value.parse().ok()?;
                Some(UserAmount {
                    user_id: *uid,
                    amount: amt.abs(),
                })
            })
            .collect();

        // Parse date from Tricount's created field
        let date = entry
            .created
            .as_deref()
            .and_then(|s| {
                // Try common formats: "2024-01-15 12:00:00" or "2024-01-15"
                chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                    .ok()
                    .or_else(|| {
                        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                            .ok()
                            .map(|dt| dt.date())
                    })
            })
            .unwrap_or_else(|| chrono::Local::now().date_naive());

        let creatable_expense = CreatableExpense {
            name: entry.description.clone(),
            amount,
            expense_type: ExpenseType::Expense,
            project_id,
            payers: vec![UserAmount {
                user_id: payer_id,
                amount,
            }],
            debtors,
            author_id: payer_id,
            description: None,
            date,
        };

        // Create expense in DB
        let expense_id = expenses_repository::add_expense(creatable_expense.clone()).await?;

        // Create payments (same logic as expenses_controller::add_expense)
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

    // 6. Return response
    let project = projects_repository::get_project(project_id).await?;

    Ok(TricountImportResponse {
        project,
        users: created_users,
        expenses_count,
    })
}
