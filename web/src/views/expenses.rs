use crate::views::{ExpenseList, ExpensesHeader, ExpensesUserSection, SummaryCard};
use crate::Route;
use api::{add_user, get_expenses_by_project_id, get_users_by_project_id};
use dioxus::prelude::*;
use shared::{CreatableUser, Expense, User};
use ui::{AddExpenseModal, AddUserModal, Avatar, BackButtonArrow};
use uuid::Uuid;

pub mod expense_item;
pub mod expense_list;
pub mod expenses_header;
pub mod expenses_user_section;
pub mod summary_card;

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesProps {
    project_id: Uuid,
}

#[component]
pub fn Expenses(props: ExpensesProps) -> Element {
    let mut users: Signal<Vec<User>> = use_signal(|| vec![]);
    let mut is_expense_modal_open = use_signal(|| false);
    let mut expenses: Signal<Vec<Expense>> = use_signal(|| vec![]);

    let _ = use_resource(move || async move {
        match get_users_by_project_id(props.project_id).await {
            Ok(u) => users.set(u),
            Err(_) => ()
        }
    });

    let _ = use_resource(move || async move {
        match get_expenses_by_project_id(props.project_id).await {
            Ok(e) => expenses.set(e),
            Err(_) => ()
        }
    });

    let global_total: f64 = expenses().iter().map(|e| e.amount).reduce(|acc, expense| acc + expense).unwrap_or(0.0);

    rsx! {
        div {
            class: "container bg-base-100 p-4 max-w-md rounded-xl flex flex-col",

            ExpensesHeader { title: "Weekend Paris" }
            ExpensesUserSection { id: props.project_id, users: users() }
            SummaryCard { my_total: 625.0, global_total }

            // Expense list
            div {
                class: "mt-6",

                // DateSeparator { label: "Today" }
                ExpenseList { expenses: expenses() }
                // DateSeparator { label: "Yesterday" }
            }
            if (users().len() > 0) {
                button {
                    class: "btn btn-circle btn-outline btn-lg bg-base-100 self-center mt-6",
                    onclick: move |_| is_expense_modal_open.set(true),
                    "+"
                },
                AddExpenseModal { is_expense_modal_open, users: users(), project_id: props.project_id }
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
        div {
            class: "divider divider-start text-primary font-bold text-sm my-4",
            "{props.label}"
        }
    }
}

