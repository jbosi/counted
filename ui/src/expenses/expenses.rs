use crate::common::AppHeader;
use crate::expenses::{ExpenseList, ExpensesUserSection, SummaryCard};
use crate::modals::AddExpenseModal;
use crate::route::Route;
use api::expenses::get_expenses_by_project_id;
use api::projects::get_project;
use api::users::get_users_by_project_id;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use shared::view_models::users_project_view_model::UsersProject;
use shared::{Expense, User};
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesProps {
    project_id: Uuid,
}

#[component]
pub fn Expenses(props: ExpensesProps) -> Element {
    let mut is_expense_modal_open = use_signal(|| false);

    // let mut users_project_context: Signal<Option<UsersProject>> =
    //     use_context_provider(|| Signal::new(None));

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
        get_expenses_by_project_id(props.project_id).await.unwrap_or_else(|_| vec![])
    });

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
                        div {
                            class: "flex justify-center",
                            span {
                                class:"loading loading-spinner loading-m"
                            }
                        }
                    }
                }
            }
            match (users_resource.read_unchecked().as_ref(), expenses_resource.read_unchecked().as_ref()) {
                (Some(users), Some(expenses)) => {
                    rsx! {
                        ExpensesUserSection { id: props.project_id, users: users.to_vec() }
                        SummaryCard { my_total: 625.0, global_total: expenses.iter().map(|e| e.amount).reduce(|acc, expense| acc + expense).unwrap_or(0.0) }

                        // Expense list
                        div { class: "mt-6",

                            // DateSeparator { label: "Today" }
                            ExpenseList { expenses: expenses.to_vec() }
                                    // DateSeparator { label: "Yesterday" }
                        }
                        if users.len() > 0 {
                            button {
                                type: "button",
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
                    }
                }
                _ => {
                    rsx! {
                        div {
                            class: "flex justify-center mt-20",
                            span {
                                class:"loading loading-spinner loading-xl"
                            }
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
