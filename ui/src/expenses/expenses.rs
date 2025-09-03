use crate::common::{AppHeader, Avatar};
use crate::expenses::{ExpenseList, ExpensesUserSection, SummaryCard};
use crate::modals::AddExpenseModal;
use crate::route::Route;
use crate::utils::listen_to_sse_events;
use api::expenses::get_expenses_by_project_id;
use api::payments::get_summary_by_user_ids;
use api::projects::get_project;
use api::users::get_users_by_project_id;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use shared::sse::EventSSE::{ExpenseCreated, ExpenseDeleted, ExpenseModified};
use shared::view_models::users_project_view_model::UsersProject;
use shared::{Expense, User};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesProps {
    project_id: Uuid,
}

#[derive(PartialEq, Clone)]
enum ActiveTab {
    ExpensesList,
    Summary,
}

#[component]
pub fn Expenses(props: ExpensesProps) -> Element {
    let mut is_expense_modal_open = use_signal(|| false);
    let mut expense_event_any = use_signal(|| String::new());
    let mut active_tab: Signal<ActiveTab> = use_signal(|| ActiveTab::ExpensesList);

    let users_resource = use_resource(move || async move {
        get_users_by_project_id(props.project_id).await.unwrap_or_else(|_| vec![])
    });

    let project_resource = use_resource(move || async move {
        match get_project(props.project_id).await {
            Ok(p) => p,
            Err(_) => panic!("Failed to find project"),
        }
    });

    let expenses_resource = use_resource(move || async move {
        // rerun the resource when event is fired
        let _ = expense_event_any();

        get_expenses_by_project_id(props.project_id).await.unwrap_or_else(|_| vec![])
    });

    let summary_resource: Resource<HashMap<i32, f64>> = use_resource(move || async move {
        let _ = expense_event_any();

        match users_resource.read_unchecked().as_ref() {
            None => HashMap::new(),
            Some(users) => get_summary_by_user_ids(users.iter().map(|u| u.id).collect())
                .await
                .unwrap_or_default(),
        }
    });

    listen_to_sse_events(
        Vec::from([ExpenseCreated, ExpenseDeleted, ExpenseModified]),
        expense_event_any,
    );

    rsx! {
        div { class: "container overflow-auto app-container bg-base-200 p-4 max-w-md rounded-xl flex flex-col",
            match project_resource.read_unchecked().as_ref() {
                Some(p) => {
                    rsx! {
                        AppHeader { title: p.name.clone(), back_button_route: Route::Projects {} }
                    }
                }
                None => {
                    rsx! {
                        div { class: "flex justify-center",
                            span { class: "loading loading-spinner loading-m" }
                        }
                    }
                }
            }
            match (
                users_resource.read_unchecked().as_ref(),
                expenses_resource.read_unchecked().as_ref(),
            ) {
                (Some(users), Some(expenses)) => {
                    rsx! {
                        ExpensesUserSection { id: props.project_id, users: users.to_vec() }
                        SummaryCard {
                            my_total: 625.0,
                            global_total: expenses
                                .iter()
                                .map(|e| e.amount)
                                .reduce(|acc, expense| acc + expense)
                                .unwrap_or(0.0),
                        }
                        div { role: "tablist", class: "tabs tabs-box justify-center",
                            a {
                                role: "tab",
                                class: "tab",
                                class: if active_tab() == ActiveTab::ExpensesList { "tab-active" },
                                onclick: move |_| { active_tab.set(ActiveTab::ExpensesList) },
                                "Liste des dépenses"
                            }
                            a {
                                role: "tab",
                                class: "tab",
                                class: if active_tab() == ActiveTab::Summary { "tab-active" },
                                onclick: move |_| { active_tab.set(ActiveTab::Summary) },
                                "Equilibre"
                            }
                        }
                        
                        if active_tab() == ActiveTab::ExpensesList {
                            // Expense list
                            div { class: "mt-6",
                        
                                // DateSeparator { label: "Today" }
                                ExpenseList { expenses: expenses.to_vec() }
                            }
                            if users.len() > 0 {
                                button {
                                    r#type: "button",
                                    class: "btn btn-circle btn-outline btn-lg sticky bottom-0 self-center mt-6",
                                    onclick: move |_| is_expense_modal_open.set(true),
                                    "+"
                                }
                                // position: absolute;
                                //   bottom: 3rem;
                                // }
                                AddExpenseModal {
                                    is_expense_modal_open,
                                    users: users.to_vec(),
                                    project_id: props.project_id,
                                }
                            }
                        } else {
                            match summary_resource.read_unchecked().as_ref() {
                                Some(summary_by_users) => {
                                    let max_amount = summary_by_users
                                        .iter()
                                        .map(|(_, a)| a.abs())
                                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                                        .unwrap_or(1.0);
                                    rsx! {
                                        section { class: "flex flex-col gap-2",
                                            for (summary_user_id , summary_amount) in summary_by_users {
                                                div { class: "flex gap-2 justify-between",
                                                    div { class: "flex gap-2",
                                                        Avatar {
                                                            initials: users
                                                                .iter()
                                                                .find(|u| u.id == *summary_user_id)
                                                                .unwrap()
                                                                .name
                                                                .get(0..2)
                                                                .unwrap_or(""),
                                                            size: 12,
                                                        }
                                                        div { class: "self-center",
                                                            span {
                                                                if summary_amount.is_sign_negative() {
                                                                    "{summary_amount} €"
                                                                } else {
                                                                    "+{summary_amount} €"
                                                                }
                                                            }
                                                        }
                                                    }
                                                    progress {
                                                        class: if *summary_amount > 0.0 { "progress progress-primary self-center" } else { "progress progress-error self-center" },
                                                        style: if *summary_amount < 0.0 { "transform: translateX(-100%);" },
                                                        style: "width: {(summary_amount.abs() * 30.0) / max_amount}%",
                                                        value: "100",
                                                        max: "100",
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => rsx! {},
                            }
                        }
                    }
                }
                _ => {
                    rsx! {
                        div { class: "flex justify-center mt-20",
                            span { class: "loading loading-spinner loading-xl" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct DateSeparatorProps {
    label: String,
}

#[component]
fn DateSeparator(props: DateSeparatorProps) -> Element {
    rsx! {
        div { class: "divider divider-start text-primary font-bold text-sm my-4", "{props.label}" }
    }
}
