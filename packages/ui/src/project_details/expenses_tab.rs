use dioxus::prelude::*;
use shared::{Expense, ExpenseType, Payment, User};
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

use crate::common::{Avatar, EmptyMagnifyingGlassIllustration};
use crate::icons::SettingsIcon;
use crate::project_details::AddExpenseModal;
use crate::route::Route;

fn get_expense_emoji(expense_name: &str) -> &'static str {
    let name = expense_name.to_lowercase();
    let has = |kws: &[&str]| kws.iter().any(|k| name.contains(*k));

    // Food & Drinks
    if has(&["restaurant", "resto", "dinner", "lunch", "breakfast", "meal", "food", "eat"]) { return "🍽️"; }
    if has(&["coffee", "café", "starbucks", "tea"]) { return "☕"; }
    if has(&["pizza"]) { return "🍕"; }
    if has(&["burger"]) { return "🍔"; }
    if has(&["sushi"]) { return "🍣"; }
    if has(&["beer", "bar", "pub", "bière", "biere", "drink", "wine", "alcohol"]) { return "🍺"; }
    if has(&["grocery", "groceries", "supermarket", "market", "food shopping", "courses"]) { return "🛒"; }
    if has(&["ice cream", "dessert"]) { return "🍦"; }
    // Transportation
    if has(&["uber", "taxi", "cab", "ride"]) { return "🚕"; }
    if has(&["gas", "fuel", "essence", "petrol"]) { return "⛽"; }
    if has(&["train", "railway"]) { return "🚆"; }
    if has(&["plane", "flight", "airplane"]) { return "✈️"; }
    if has(&["bus"]) { return "🚌"; }
    if has(&["car", "vehicle", "auto"]) { return "🚗"; }
    if has(&["bike", "bicycle"]) { return "🚲"; }
    if has(&["parking"]) { return "🅿️"; }
    // Accommodation
    if has(&["hotel", "airbnb", "accommodation", "lodging"]) { return "🏨"; }
    if has(&["rent", "loyer"]) { return "🏠"; }
    // Entertainment
    if has(&["movie", "cinema", "film"]) { return "🎬"; }
    if has(&["concert", "music", "festival"]) { return "🎵"; }
    if has(&["game", "gaming"]) { return "🎮"; }
    if has(&["ski", "skiing", "snowboard"]) { return "🎿"; }
    if has(&["sport", "gym", "fitness"]) { return "⚽"; }
    if has(&["ticket", "billet"]) { return "🎟️"; }
    // Shopping
    if has(&["shop", "shopping", "clothes", "clothing", "fashion"]) { return "🛍️"; }
    if has(&["phone", "mobile", "smartphone"]) { return "📱"; }
    if has(&["computer", "laptop"]) { return "💻"; }
    if has(&["book", "library"]) { return "📚"; }
    // Services
    if has(&["internet", "wifi"]) { return "📡"; }
    if has(&["electricity", "electric"]) { return "⚡"; }
    if has(&["water", "eau"]) { return "💧"; }
    if has(&["insurance", "assurance"]) { return "🛡️"; }
    if has(&["medical", "doctor", "hospital", "health", "pharmacy"]) { return "🏥"; }
    if has(&["haircut", "salon", "coiffeur"]) { return "💇"; }
    if has(&["spa", "massage", "wellness"]) { return "💆"; }
    // Gifts & Special
    if has(&["gift", "cadeau", "present"]) { return "🎁"; }
    if has(&["birthday", "anniversaire"]) { return "🎂"; }
    if has(&["christmas", "noël"]) { return "🎄"; }

    "💵"
}

fn expense_type_label(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Expense => "Dépense",
        ExpenseType::Transfer => "Transfert",
        ExpenseType::Gain => "Gain",
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesTabProps {
    pub expenses: Vec<Expense>,
    pub payments: Vec<Payment>,
    pub stored_user_id: Option<i32>,
    pub project_id: Uuid,
    pub currency: String,
    pub users: Vec<User>,
    pub on_expense_created: EventHandler<()>,
}

#[component]
pub fn ExpensesTab(props: ExpensesTabProps) -> Element {
    let nav = use_navigator();
    let mut show_my_payments = use_signal(|| false);
    let mut show_my_debts = use_signal(|| false);
    let mut show_add_expense = use_signal(|| false);

    let filtered_expenses = move || -> Vec<Expense> {
        let uid = match props.stored_user_id {
            Some(id) if show_my_payments() || show_my_debts() => id,
            _ => return props.expenses.clone(),
        };

        let expense_ids: HashSet<i32> = props
            .payments
            .iter()
            .filter(|p| {
                p.user_id == uid
                    && ((show_my_payments() && !p.is_debt) || (show_my_debts() && p.is_debt))
            })
            .map(|p| p.expense_id)
            .collect();

        props.expenses.iter().filter(|e| expense_ids.contains(&e.id)).cloned().collect()
    };

    // Group by date, sorted descending
    let grouped = move || -> Vec<(chrono::NaiveDate, Vec<Expense>)> {
        let mut map: BTreeMap<chrono::NaiveDate, Vec<Expense>> = BTreeMap::new();
        for e in filtered_expenses() {
            map.entry(e.date).or_default().push(e);
        }
        let mut groups: Vec<_> = map.into_iter().collect();
        groups.sort_by(|(a, _), (b, _)| b.cmp(a));
        groups
    };

    let groups = grouped();
    let is_empty = groups.is_empty();
    let project_id = props.project_id;
    let currency = props.currency.clone();

    rsx! {
        div { class: "flex flex-col gap-2",
            // Filter toolbar
            div { class: "flex justify-end",
                details { class: "dropdown dropdown-end",
                    summary { class: "btn btn-ghost btn-circle btn-sm", SettingsIcon {} }
                    ul { class: "menu dropdown-content bg-base-100 rounded-box w-56 shadow z-10 p-2",
                        li {
                            label { class: "flex items-center gap-3 cursor-pointer",
                                span { class: "flex-1 text-sm", "Mes paiements" }
                                input {
                                    r#type: "checkbox",
                                    class: "toggle toggle-sm",
                                    checked: show_my_payments(),
                                    oninput: move |e| show_my_payments.set(e.checked()),
                                }
                            }
                        }
                        li {
                            label { class: "flex items-center gap-3 cursor-pointer",
                                span { class: "flex-1 text-sm", "Mes dettes" }
                                input {
                                    r#type: "checkbox",
                                    class: "toggle toggle-sm",
                                    checked: show_my_debts(),
                                    oninput: move |e| show_my_debts.set(e.checked()),
                                }
                            }
                        }
                    }
                }
            }

            if is_empty {
                div { class: "flex flex-col items-center gap-2 py-12 text-base-content/60",
                    EmptyMagnifyingGlassIllustration {}
                    span { class: "font-semibold", "Aucune dépense" }
                    span { class: "text-sm text-center",
                        "Commencez par ajouter des dépenses en cliquant sur le bouton ci-dessous"
                    }
                }
            } else {
                for (date , group) in groups {
                    div { class: "flex flex-col",
                        div { class: "divider divider-start text-xs text-base-content/60 font-medium my-1",
                            "{date.format(\"%d/%m/%Y\")}"
                        }
                        ul { class: "flex flex-col gap-1",
                            for expense in group {
                                {
                                    let expense_id = expense.id;
                                    let amount = expense.amount;
                                    let name = expense.name.clone();
                                    let etype = expense.expense_type.clone();
                                    let curr = currency.clone();
                                    let emoji = get_expense_emoji(&name);
                                    rsx! {
                                    li {
                                        class: "flex items-center gap-3 p-3 bg-base-100 rounded-lg shadow-sm cursor-pointer hover:bg-base-200 transition-colors",
                                        onclick: move |_| {
                                            nav.push(Route::PaymentPage {
                                                project_id,
                                                expense_id,
                                            });
                                        },
                                        Avatar {
                                            initials: emoji.to_string(),
                                            size: 10,
                                            color_class: "bg-transparent".to_string(),
                                        }
                                        div { class: "flex-1 min-w-0",
                                            p { class: "font-medium truncate", "{name}" }
                                            p { class: "text-xs text-base-content/60", "{expense_type_label(&etype)}" }
                                        }
                                        div { class: "text-right shrink-0",
                                            p { class: "text-sm font-semibold", "{amount:.2} {curr}" }
                                        }
                                    }
                                }
                                }
                            }
                        }
                    }
                }
            }

            // FAB
            div { class: "fixed bottom-6 right-6",
                button {
                    r#type: "button",
                    class: "btn btn-circle btn-lg btn-primary shadow-lg",
                    "aria-label": "Ajouter une dépense",
                    onclick: move |_| {
                        show_add_expense.set(true);
                    },
                    "+"
                }
            }

            if show_add_expense() {
                AddExpenseModal {
                    on_close: move |_| show_add_expense.set(false),
                    on_created: move |_| {
                        show_add_expense.set(false);
                        props.on_expense_created.call(());
                    },
                    project_id: props.project_id,
                    users: props.users.clone(),
                    stored_user_id: props.stored_user_id,
                }
            }
        }
    }
}
