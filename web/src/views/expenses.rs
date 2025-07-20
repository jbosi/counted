use dioxus::html::a::size;
use crate::dioxus_elements::button::r#type;
use crate::Route;
use api::{add_user, get_expenses_by_project_id, get_users_by_project_id};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use shared::{CreatableUser, Expense, User};
use ui::{AddExpenseModal, AddUserModal, Avatar, BackButtonArrow};
use uuid::Uuid;


// TODO rename into project_details
#[derive(PartialEq, Props, Clone)]
pub struct ExpensesProps {
    id: Uuid,
}

#[component]
pub fn Expenses(props: ExpensesProps) -> Element {
    let mut users: Signal<Vec<User>> = use_signal(|| vec![]);
    let mut is_expense_modal_open = use_signal(|| false);
    let mut expenses: Signal<Vec<Expense>> = use_signal(|| vec![]);

    let _ = use_resource(move || async move {
        match get_users_by_project_id(props.id).await {
            Ok(u) => users.set(u),
            Err(_) => ()
        }
    });

    let _ = use_resource(move || async move {
        match get_expenses_by_project_id(props.id).await {
            Ok(e) => expenses.set(e),
            Err(_) => ()
        }
    });

    let global_total: f64 = expenses().iter().map(|e| e.amount).reduce(|acc, expense| acc + expense).unwrap_or(0.0);

    rsx! {
        div {
            class: "container bg-base-100 p-4 max-w-md rounded-xl flex flex-col",

            Header { title: "Weekend Paris" }
            UserSection { id: props.id, users: users() }
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
                AddExpenseModal { is_expense_modal_open, users: users(), project_id: props.id }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct HeaderProps {
    title: String,
}

fn Header(props: HeaderProps) -> Element {
    rsx! {
        div {
            class: "navbar px-0",
            div {
                class: "navbar-start",
                onclick: move |_| {
                    navigator().push(Route::Projects {});
                },
                BackButtonArrow {},
            }
            div {
                class: "navbar-center",
                h1 { class: "text-xl font-bold", "{props.title}" }
            }
            div {
                class: "navbar-end",
                button { 
                    class: "btn btn-ghost btn-circle",
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        view_box: "0 0 24 24",
                        path { d: "M3 12h18M3 6h18M3 18h18" }
                    },
                }   
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct UserSectionProps {
    id: Uuid,
    users: Vec<User>,
}
fn UserSection(props: UserSectionProps) -> Element {
    let mut is_user_modal_open = use_signal(|| false);
    rsx! {
        div {
            class: "flex justify-between items-center my-6 p-4",
            
            button {
                class: "btn btn-circle btn-outline btn-lg",
                onclick: move |_| is_user_modal_open.set(true),
                "+"
            }
            
            div {
                class: "avatar-group -space-x-4",
                for user in props.users {
                    Avatar { initials: user.name.get(0..2).unwrap_or(""), size: 12 }
                }
            }
            
            div { class: "w-16" }
        }
        AddUserModal { is_user_modal_open, id: props.id }
    }
}

#[derive(PartialEq, Props, Clone)]
struct SummaryCardProps {
    my_total: f64,
    global_total: f64,
}

fn SummaryCard(props: SummaryCardProps) -> Element {
    let format_currency = |val: f64| format!("{:.2}€", val).replace('.', ",");

    rsx! {
        div {
            class: "",
            div {
                class: "card-body p-4 space-y-3",
                
                div {
                    class: "flex justify-between items-center",
                    span { class: "text-base-content", "Mon Total" }
                    span { class: "font-bold text-lg", "{format_currency(props.my_total)}" }
                }
                
                div {
                    class: "flex justify-between items-center",
                    span { class: "text-base-content/70", "Total global" }
                    span { class: "font-semibold text-lg text-base-content/70", "{format_currency(props.global_total)}" }
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
        div {
            class: "divider divider-start text-primary font-bold text-sm my-4",
            "{props.label}"
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct ExpenseListProps {
    expenses: Vec<Expense>,
}

fn ExpenseList(props: ExpenseListProps) -> Element {
    rsx! {
        div {
            for expense in props.expenses {
                ExpenseItem {
                    key: "{expense.id}",
                    expense: expense.clone()
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct ExpenseItemProps {
    expense: Expense,
}

fn ExpenseItem(props: ExpenseItemProps) -> Element {
    let expense: &Expense = &props.expense;
    let formatted_amount = format!("{:.2}€", expense.amount).replace('.', ",");

    rsx! {
        div {
            class: "flex items-center gap-4 p-3 hover:bg-base-200 rounded-lg transition-colors",

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

