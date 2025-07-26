use crate::common::{AppHeader, Avatar, BackButtonArrow};
use crate::expenses::{ExpenseList, ExpensesUserSection, SummaryCard};
use crate::modals::{AddExpenseModal, AddUserModal};
use crate::route::Route;
use api::{add_user, get_expenses_by_project_id, get_project, get_users_by_project_id};
use dioxus::prelude::*;
use shared::{CreatableUser, Expense, User};
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesProps {
    project_id: Uuid,
}

#[component]
pub fn Expenses(props: ExpensesProps) -> Element {
    let mut users: Signal<Vec<User>> = use_signal(|| vec![]);
    let mut is_expense_modal_open = use_signal(|| false);
    let mut expenses: Signal<Vec<Expense>> = use_signal(|| vec![]);

    use_resource(move || async move {
        match get_users_by_project_id(props.project_id).await {
            Ok(u) => users.set(u),
            Err(_) => ()
        }
    });

    use_resource(move || async move {
        match get_expenses_by_project_id(props.project_id).await {
            Ok(e) => expenses.set(e),
            Err(_) => ()
        }
    });

    let project_resource = use_resource(move || async move {
        get_project(props.project_id).await
    });

    let global_total: f64 = expenses().iter().map(|e| e.amount).reduce(|acc, expense| acc + expense).unwrap_or(0.0);

    rsx! {
        div { class: "container bg-base-100 p-4 max-w-md rounded-xl flex flex-col",

            if let Some(project) = &*project_resource.read() {
                match project {
                    Ok(p) => rsx! {
                        AppHeader { title: p.name.clone(), back_button_route: Route::Projects {} }
                    },
                    Err(err) => rsx! {
                    "Failed to fetch response: {err}"
                    },
                }
            }
            ExpensesUserSection { id: props.project_id, users: users() }
            SummaryCard { my_total: 625.0, global_total }

            // Expense list
            div { class: "mt-6",

                // DateSeparator { label: "Today" }
                ExpenseList { expenses: expenses() }
                        // DateSeparator { label: "Yesterday" }
            }
            if (users().len() > 0) {
                button {
                    class: "btn btn-circle btn-outline btn-lg bg-base-100 self-center mt-6",
                    onclick: move |_| is_expense_modal_open.set(true),
                    "+"
                }
                AddExpenseModal {
                    is_expense_modal_open,
                    users: users(),
                    project_id: props.project_id,
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct DateSeparatorProps {
    label: String,
}

fn DateSeparator(props: DateSeparatorProps) -> Element {
    rsx! {
        div { class: "divider divider-start text-primary font-bold text-sm my-4", "{props.label}" }
    }
}

