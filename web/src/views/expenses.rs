use crate::Route;
use dioxus::prelude::*;
use uuid::Uuid;
use api::{get_users_by_project_id, add_user};
use shared::{User};

// TODO rename into project_details
#[component]
pub fn Expenses(id: Uuid) -> Element {
    let mut users: Signal<Vec<User>> = use_signal(|| vec![]);

    let _ = use_resource(move || async move {
        match get_users_by_project_id(id).await {
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
            UserSection {}
            SummaryCard { my_total: 625.0, global_total: 3200.0 }

            // Liste des transactions
            div {
                class: "mt-6",

                DateSeparator { label: "Today" }
                TransactionList { transactions: today_transactions }

                DateSeparator { label: "Yesterday" }
                TransactionList { transactions: yesterday_transactions }
            }
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
                // Espace vide pour centrer le titre
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

fn UserSection() -> Element {
    let mut modal_open = use_signal(|| false);
    rsx! {
        div {
            class: "flex justify-between items-center my-6",
            
            button {
                class: "btn btn-circle btn-outline btn-lg",
                onclick: move |_| modal_open.set(true),
                "+"
            }
            
            div {
                class: "avatar-group -space-x-4",
                Avatar { initials: "MA" }
                Avatar { initials: "TE" }
                Avatar { initials: "BU" }
            }
            
            // Espace vide pour équilibrer
            div { class: "w-16" }
        }
        AddUserModal { modal_open }
    }
}

#[derive(PartialEq, Props, Clone)]
struct AvatarProps {
    initials: String,
}

fn Avatar(props: AvatarProps) -> Element {
    rsx! {
        div {
            class: "avatar placeholder",
            div {
                class: "bg-neutral text-neutral-content rounded-full w-12",
                span { class: "text-sm font-bold", "{props.initials}" }
            }
        }
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
            {props.transactions.iter().map(|tx| rsx! {
                TransactionItem {
                    key: "{tx.id}",
                    transaction: tx.clone()
                }
            })}
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

#[component]
fn AddUserModal(modal_open: Signal<bool>) -> Element {    
    rsx! {
        dialog {
            id: "add_user_modal",
            class: "modal",
            class: if modal_open() { "modal-open" } else { "" },
            div {
                class: "modal-box",
                h3 {
                    class: "text-lg font-bold",
                    "Hello!"
                }
                p {
                    class: "py-4",
                    "Press ESC key or click outside to close"
                }
            }
            form {
                method: "dialog",
                class: "modal-backdrop",
                onclick: move |_| modal_open.set(false),
                button {
                    "close"
                }
            }
        }
    }
}