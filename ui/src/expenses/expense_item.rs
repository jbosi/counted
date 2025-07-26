use crate::route::Route;
use crate::Avatar;
use dioxus::prelude::*;
use shared::{Expense, User};
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct ExpenseItemProps {
    expense: Expense,
}

pub fn ExpenseItem(props: ExpenseItemProps) -> Element {
    let expense: &Expense = &props.expense;
    let formatted_amount = format!("{:.2}€", expense.amount).replace('.', ",");

    rsx! {
        div {
            class: "flex items-center gap-4 p-3 hover:bg-base-200 rounded-lg transition-colors",
            onclick: move |_| {
                navigator().push(Route::Payments { project_id: props.expense.project_id, expense_id: props.expense.id });
            },
            // Category
            Avatar { initials: "💰", size: 10 }

            // Name
            div {
                class: "flex-1 min-w-0",
                p {
                    class: "font-semibold text-base-content truncate",
                    "{expense.name}"
                }
            }

            // Amount
            div {
                class: "text-right",
                p {
                    class: "font-bold text-lg text-base-content",
                    "{formatted_amount}"
                }
            }
        }
    }
}