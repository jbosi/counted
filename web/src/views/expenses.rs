use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Expenses(id: i32) -> Element {
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
            class: "p-4",

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
        header {
            class: "flex justify-between items-center py-4",
            // Un espace vide pour pousser le titre au centre
            span { class: "w-8" }
            h1 { class: "text-2xl font-semibold text-gray-800", "{props.title}" }
            // Icône de menu Hamburger
            svg {
                class: "w-7 h-7 text-gray-600",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
                view_box: "0 0 24 24",
                path { d: "M3 12h18M3 6h18M3 18h18" }
            }
        }
    }
}

fn UserSection() ->Element {
    rsx! {
        section {
            class: "flex justify-between items-center my-6 px-2",
            button {
                class: "flex items-center justify-center w-14 h-14 bg-gray-100 rounded-full text-3xl text-gray-500 font-light",
                "+"
            }
            div {
                class: "flex space-x-2",
                Avatar { initials: "MA" }
                Avatar { initials: "TE" }
                Avatar { initials: "BU" }
            }
            // Espace vide pour équilibrer le flex
            span { class: "w-14" }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct AvatarProps {
    initials: String,
}

fn Avatar(props: AvatarProps) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center w-12 h-12 bg-gray-400 text-white rounded-full font-bold text-lg",
            "{props.initials}"
        }
    }
}


#[derive(PartialEq, Props, Clone)]
struct SummaryCardProps {
    my_total: f32,
    global_total: f32,
}
fn SummaryCard(props: SummaryCardProps) -> Element {
    // Helper pour formater les nombres avec une virgule
    let format_currency = |val: f32| format!("{:.2}€", val).replace('.', ",");

    rsx! {
        div {
            class: "bg-gray-50 rounded-lg p-4 space-y-2",
            div {
                class: "flex justify-between items-center text-gray-800",
                p { "Mon Total" }
                p { class: "font-semibold text-lg", "{format_currency(props.my_total)}" }
            }
            div {
                class: "flex justify-between items-center text-gray-500",
                p { "Total global" }
                p { class: "font-semibold text-lg", "{format_currency(props.global_total)}" }
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
        h3 {
            class: "text-blue-500 font-bold text-sm my-4",
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
            class: "space-y-4",
            {props.transactions.iter().map(|tx| rsx! {
                TransactionItem {
                    key: "{tx.id}", // La clé est importante pour Dioxus
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
            class: "flex items-center space-x-4",
            // Icône de catégorie
            div { class: "w-11 h-11 bg-gray-200 rounded-full flex-shrink-0" }
            // Détails de la transaction
            div {
                p { class: "font-semibold text-gray-800", "{tx.category}" }
                p { class: "text-sm text-gray-500", "{tx.paid_by}" }
            }
            // Montant (aligné à droite)
            p {
                class: "ml-auto font-semibold text-lg text-gray-800",
                "{formatted_amount}"
            }
        }
    }
}