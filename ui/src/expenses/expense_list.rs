use crate::expenses::ExpenseItem;
use dioxus::prelude::*;
use shared::{Expense, User};

#[derive(PartialEq, Props, Clone)]
pub struct ExpenseListProps {
    expenses: Vec<Expense>,
}

pub fn ExpenseList(props: ExpenseListProps) -> Element {
    rsx! {
        div {
            for expense in props.expenses {
                ExpenseItem {
                    key: "{expense.id}",
                    expense: expense.clone(),
                }
            }
        }
    }
}