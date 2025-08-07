use crate::expenses::ExpenseItem;
use dioxus::prelude::*;
use shared::Expense;

#[derive(PartialEq, Props, Clone)]
pub struct ExpenseListProps {
    expenses: Vec<Expense>,
}

#[component]
pub fn ExpenseList(props: ExpenseListProps) -> Element {
    rsx! {
        div {
            for expense in props.expenses {
                ExpenseItem { key: "{expense.id}", expense: expense.clone() }
            }
        }
    }
}
