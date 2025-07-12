use dioxus::html::a::size;
use crate::dioxus_elements::button::r#type;
use crate::Route;
use api::{add_user, get_users_by_project_id};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use shared::{CreatableUser, User};
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

    let _ = use_resource(move || async move {
        match get_users_by_project_id(props.id).await {
            Ok(u) => users.set(u),
            Err(_) => ()
        }
    });
    let today_transactions = vec![
        Transaction { id: 1, category: "Repas".to_string(), paid_by: "Payé par Rober".to_string(), amount: 50.0 },
        Transaction { id: 2, category: "Piscine de Paris".to_string(), paid_by: "Payé par Léo".to_string(), amount: 12.99 },
    ];

    let yesterday_transactions = vec![
        Transaction { id: 3, category: "Repas".to_string(), paid_by: "Payé par Rober".to_string(), amount: 50.0 },
        Transaction { id: 4, category: "Piscine".to_string(), paid_by: "Payé par Léo et Louise".to_string(), amount: 12.99 },
        Transaction { id: 5, category: "Piscine".to_string(), paid_by: "Payé par Téo et Bertrant".to_string(), amount: 50.99 },
        Transaction { id: 6, category: "Piscine".to_string(), paid_by: "Payé par Léo".to_string(), amount: 12.99 },
        Transaction { id: 7, category: "Piscine".to_string(), paid_by: "Payé par Léo".to_string(), amount: 12.99 },
    ];

    rsx! {
        div {
            class: "container bg-base-100 mx-auto p-4 max-w-md rounded-xl",

            Header { title: "Weekend Paris" }
            UserSection { id: props.id, users: users() }
            SummaryCard { my_total: 625.0, global_total: 3200.0 }

            // Liste des transactions
            div {
                class: "mt-6",

                DateSeparator { label: "Today" }
                TransactionList { transactions: today_transactions }

                DateSeparator { label: "Yesterday" }
                TransactionList { transactions: yesterday_transactions }
            }
            button {
                class: "btn btn-circle btn-outline btn-lg bg-base-100",
                onclick: move |_| is_expense_modal_open.set(true),
                "+"
            },
            AddExpenseModal { is_expense_modal_open, users: users() }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Transaction {
    id: u32,
    category: String,
    paid_by: String,
    amount: f32,
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
            class: "flex justify-between items-center my-6",
            
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
    my_total: f32,
    global_total: f32,
}

fn SummaryCard(props: SummaryCardProps) -> Element {
    let format_currency = |val: f32| format!("{:.2}€", val).replace('.', ",");

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
struct TransactionListProps {
    transactions: Vec<Transaction>,
}

fn TransactionList(props: TransactionListProps) -> Element {
    rsx! {
        div {
            class: "space-y-3",
            {
                props.transactions.iter().map(|tx| rsx! {
                    TransactionItem {
                        key: "{tx.id}",
                        transaction: tx.clone()
                    }
                })
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct TransactionItemProps {
    transaction: Transaction,
}

fn TransactionItem(props: TransactionItemProps) -> Element {
    let tx = &props.transaction;
    let formatted_amount = format!("{:.2}€", tx.amount).replace('.', ",");

    rsx! {
        div {
            class: "flex items-center gap-4 p-3 hover:bg-base-200 rounded-lg transition-colors",
            
            // Icône de catégorie
            div {
                class: "avatar placeholder",
                div {
                    class: "bg-base-300 text-base-content rounded-full w-11",
                    span { class: "text-lg", "💰" }
                }
            }
            
            // Détails de la transaction
            div {
                class: "flex-1 min-w-0",
                p { class: "font-semibold text-base-content truncate", "{tx.category}" }
                p { class: "text-sm text-base-content/70 truncate", "{tx.paid_by}" }
            }
            
            // Montant
            div {
                class: "text-right",
                p { class: "font-bold text-lg text-base-content", "{formatted_amount}" }
            }
        }
    }
}

