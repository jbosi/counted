use api::expenses::expenses_controller::get_expenses_by_project_id;
use api::payments::payments_controller::{get_payments_by_project_id, get_summary_by_project_id};
use api::projects::projects_controller::get_project;
use api::users::users_controller::get_users_by_project_id;
use dioxus::prelude::*;
use shared::{ExpenseType, ProjectStatus, ReimbursementSuggestion, User};
use uuid::Uuid;

use crate::common::{initials, user_color_class, AppHeader, Avatar, LocalStorageState};
use crate::project_details::{AddExpenseModal, BalanceTab, ExpensesTab, ReimbursementsTab};
use crate::route::Route;

#[derive(PartialEq, Clone)]
enum Tab {
    Expenses,
    Balance,
    Reimbursements,
}

#[component]
pub fn ProjectDetails(project_id: Uuid) -> Element {
    let nav = use_navigator();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut active_tab = use_signal(|| Tab::Expenses);
    let mut show_transfer_modal = use_signal(|| false);
    // (name, amount, payer_user_id [=debtor in suggestion], debtor_user_id [=payer in suggestion])
    let mut transfer_preset: Signal<Option<(String, f64, i32, i32)>> = use_signal(|| None);

    let stored_user_id = move || {
        ls_ctx().projects.iter().find(|p| p.project_id == project_id).and_then(|p| p.user_id)
    };

    let project = use_resource(move || async move { get_project(project_id).await });
    let users = use_resource(move || async move { get_users_by_project_id(project_id).await });
    let mut expenses =
        use_resource(move || async move { get_expenses_by_project_id(project_id).await });
    let mut payments =
        use_resource(move || async move { get_payments_by_project_id(project_id).await });
    let mut summary =
        use_resource(move || async move { get_summary_by_project_id(project_id).await });

    rsx! {
        div { class: "container overflow-auto p-4 max-w-md w-full mx-auto flex flex-col gap-4 pb-24",

            // Header — waits for project
            match &*project.read() {
                None => rsx! {
                div { class: "flex justify-center py-4",
                    span { class: "loading loading-spinner loading-md" }
                }
            },
                Some(Err(e)) => rsx! {
                div { class: "alert alert-error", "{e}" }
            },
                Some(Ok(p)) => {
                    let project_name = p.name.clone();
                    let is_archived = p.status == ProjectStatus::Archived;
                    rsx! {
                AppHeader { back_button_route: Route::ProjectsList {}, title: {project_name.clone()} }
            }
                }
            }

            // Users + summary — waits for users + expenses + payments
            match (&*users.read(), &*expenses.read(), &*payments.read()) {
                (Some(Ok(user_list)), Some(Ok(expense_list)), Some(Ok(payment_list))) => {
                    let currency = project
                        .read()
                        .as_ref()
                        .and_then(|r| r.as_ref().ok())
                        .map(|p| p.currency.clone())
                        .unwrap_or_else(|| "EUR".to_string());
                    let global_total: f64 = expense_list
                        .iter()
                        .filter(|e| e.expense_type != ExpenseType::Transfer)
                        .fold(
                            0.0, // Which user are you — read-only banner when not yet selected
                            |acc, e| match e.expense_type {
                                ExpenseType::Expense => acc + e.amount,
                                ExpenseType::Gain => acc - e.amount,
                                _ => acc,
                            },
                        );
                    let uid = stored_user_id();
                    let user_list_c = user_list.clone();
                    let expense_list_c = expense_list.clone();
                    let payment_list_c = payment_list.clone();
                    let currency_c = currency.clone();
                    rsx! {
                // Which user are you — read-only banner when not yet selected
                if uid.is_none() {
                    div { class: "alert alert-info text-sm",
                        svg {
                            class: "w-4 h-4 shrink-0",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            view_box: "0 0 24 24",
                            circle { cx: "12", cy: "12", r: "10" }
                            path { d: "M12 16v-4M12 8h.01" }
                        }
                        span { "Sélection de votre profil disponible prochainement" }
                    }
                }

                // Participants
                div { class: "flex justify-center",
                    AvatarRow { users: user_list_c.clone() }
                }

                // Summary card
                div { class: "card bg-base-100 shadow-sm",
                    div { class: "card-body p-4",
                        div { class: "flex justify-between items-center",
                            span { class: "text-sm text-base-content/60", "Total des dépenses" }
                            span { class: "font-bold", "{global_total:.2} {currency_c}" }
                        }
                    }
                }

                // Tab bar
                div { role: "tablist", class: "tabs tabs-box bg-base-300",
                    button {
                        role: "tab",
                        class: if *active_tab.read() == Tab::Expenses { "tab tab-active text-xs" } else { "tab text-xs" },
                        onclick: move |_| active_tab.set(Tab::Expenses),
                        "Dépenses"
                    }
                    button {
                        role: "tab",
                        class: if *active_tab.read() == Tab::Balance { "tab tab-active text-xs" } else { "tab text-xs" },
                        onclick: move |_| active_tab.set(Tab::Balance),
                        "Équilibre"
                    }
                    button {
                        role: "tab",
                        class: if *active_tab.read() == Tab::Reimbursements { "tab tab-active text-xs" } else { "tab text-xs" },
                        onclick: move |_| active_tab.set(Tab::Reimbursements),
                        "Remboursements"
                    }
                }

                // Tab content
                match *active_tab.read() {
                    Tab::Expenses => rsx! {
                    ExpensesTab {
                        expenses: expense_list_c,
                        payments: payment_list_c,
                        stored_user_id: uid,
                        project_id,
                        currency: currency.clone(),
                        users: user_list_c.clone(),
                        on_expense_created: move |_| {
                            expenses.restart();
                            payments.restart();
                            summary.restart();
                        },
                    }
                },
                    Tab::Balance => {
                        match &*summary.read() {
                            None => rsx! {
                    div { class: "flex justify-center py-8",
                        span { class: "loading loading-spinner loading-md" }
                    }
                },
                            Some(Err(e)) => rsx! {
                    div { class: "alert alert-error", "{e}" }
                },
                            Some(Ok(s)) => rsx! {
                    BalanceTab {
                        summary: s.clone(),
                        users: user_list_c.clone(),
                        currency: currency.clone(),
                    }
                },
                        }
                    }
                    Tab::Reimbursements => {
                        match &*summary.read() {
                            None => rsx! {
                    div { class: "flex justify-center py-8",
                        span { class: "loading loading-spinner loading-md" }
                    }
                },
                            Some(Err(e)) => rsx! {
                    div { class: "alert alert-error", "{e}" }
                },
                            Some(Ok(s)) => {
                                let users_for_cb = user_list_c.clone();
                                rsx! {
                    ReimbursementsTab {
                        suggestions: s.reimbursement_suggestions.clone(),
                        users: user_list_c.clone(),
                        currency: currency.clone(),
                        on_reimburse: move |s: ReimbursementSuggestion| {
                            let debtor = users_for_cb.iter().find(|u| u.id == s.user_id_debtor);
                            let payer  = users_for_cb.iter().find(|u| u.id == s.user_id_payer);
                            if let (Some(d), Some(p)) = (debtor, payer) {
                                let name = format!("Remboursement {} vers {}", d.name, p.name);
                                transfer_preset.set(Some((name, s.amount, s.user_id_debtor, s.user_id_payer)));
                                show_transfer_modal.set(true);
                            }
                        },
                    }
                }},
                        }
                    }
                }

                // Transfer modal opened from a reimbursement suggestion
                if show_transfer_modal() {
                    if let Some((name, amount, payer_id, debtor_id)) = transfer_preset() {
                        AddExpenseModal {
                            on_close: move |_| { show_transfer_modal.set(false); transfer_preset.set(None); },
                            on_created: move |_| {
                                show_transfer_modal.set(false);
                                transfer_preset.set(None);
                                expenses.restart();
                                payments.restart();
                                summary.restart();
                            },
                            project_id,
                            users: user_list_c.clone(),
                            stored_user_id: uid,
                            initial_name: Some(name),
                            initial_amount: Some(amount),
                            initial_expense_type: Some(ExpenseType::Transfer),
                            initial_payer_id: Some(payer_id),
                            initial_debtor_id: Some(debtor_id),
                        }
                    }
                }
            }
                }
                (Some(Err(e)), _, _) | (_, Some(Err(e)), _) | (_, _, Some(Err(e))) => {
                    rsx! {
                div { class: "alert alert-error", "{e}" }
            }
                }
                _ => rsx! {
                div { class: "flex justify-center py-8",
                    span { class: "loading loading-spinner loading-md" }
                }
            },
            }
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarRow — shows up to 4 user avatars
// ---------------------------------------------------------------------------

#[derive(PartialEq, Props, Clone)]
struct AvatarRowProps {
    users: Vec<User>,
}

#[component]
fn AvatarRow(props: AvatarRowProps) -> Element {
    const MAX: usize = 4;
    let shown: Vec<&User> = props.users.iter().take(MAX).collect();
    let overflow = props.users.len().saturating_sub(MAX);

    rsx! {
        div { class: "avatar-group -space-x-3",
            for user in shown {
                Avatar {
                    initials: initials(&user.name),
                    size: 10,
                    color_class: user_color_class(user.id).to_string(),
                }
            }
            if overflow > 0 {
                div { class: "avatar avatar-placeholder",
                    div { class: "bg-neutral-focus text-neutral-content w-10 rounded-full",
                        span { class: "text-xs", "+{overflow}" }
                    }
                }
            }
        }
    }
}
