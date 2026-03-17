use dioxus::prelude::*;
use shared::{Expense, ExpenseType, Payment};
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

use crate::route::Route;

fn expense_type_label(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Expense  => "Dépense",
        ExpenseType::Transfer => "Transfert",
        ExpenseType::Gain     => "Gain",
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesTabProps {
    pub expenses: Vec<Expense>,
    pub payments: Vec<Payment>,
    pub stored_user_id: Option<i32>,
    pub project_id: Uuid,
    pub currency: String,
}

#[component]
pub fn ExpensesTab(props: ExpensesTabProps) -> Element {
    let nav = use_navigator();
    let mut show_my_payments = use_signal(|| false);
    let mut show_my_debts = use_signal(|| false);

    let filtered_expenses = move || -> Vec<Expense> {
        let uid = match props.stored_user_id {
            Some(id) if show_my_payments() || show_my_debts() => id,
            _ => return props.expenses.clone(),
        };

        let expense_ids: HashSet<i32> = props.payments.iter()
            .filter(|p| {
                p.user_id == uid
                    && ((show_my_payments() && !p.is_debt) || (show_my_debts() && p.is_debt))
            })
            .map(|p| p.expense_id)
            .collect();

        props.expenses.iter()
            .filter(|e| expense_ids.contains(&e.id))
            .cloned()
            .collect()
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
                    summary { class: "btn btn-ghost btn-circle btn-sm",
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            view_box: "0 0 24 24",
                            circle { cx: "12", cy: "12", r: "3" }
                            path { d: "M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" }
                        }
                    }
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
                    svg {
                        class: "w-12 h-12",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "1.5",
                        view_box: "0 0 24 24",
                        path { d: "M21 21l-6-6m2-5a7 7 0 1 1-14 0 7 7 0 0 1 14 0z" }
                    }
                    span { class: "font-semibold", "Aucune dépense" }
                    span { class: "text-sm text-center", "Commencez par ajouter des dépenses en cliquant sur le bouton ci-dessous" }
                }
            } else {
                for (date, group) in groups {
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
                                    rsx! {
                                        li {
                                            class: "flex items-center gap-3 p-3 bg-base-100 rounded-lg shadow-sm cursor-pointer hover:bg-base-200 transition-colors",
                                            onclick: move |_| { nav.push(Route::PaymentPage { project_id, expense_id }); },
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

            // FAB — disabled until Add Expense modal is implemented
            div { class: "fixed bottom-6 right-6",
                button {
                    r#type: "button",
                    class: "btn btn-circle btn-lg btn-primary btn-disabled shadow-lg",
                    "aria-label": "Ajouter une dépense",
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        view_box: "0 0 24 24",
                        path { d: "M12 5v14M5 12h14" }
                    }
                }
            }
        }
    }
}
