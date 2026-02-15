use crate::common::{CalloutComponent, CalloutComponentTypes};
use api::expenses::expenses_controller::add_expense;
use chrono::{Local, NaiveDate, NaiveDateTime};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use shared::{CreatableExpense, ExpenseType, User, UserAmount};
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct AddExpenseModalProps {
    project_id: Uuid,
    is_expense_modal_open: Signal<bool>,
    users: Vec<User>,
}

#[derive(Clone, Debug, PartialEq)]
struct FormCheckboxAmount {
    label: String,
    user_id: i32,
    is_checked: bool,
    amount: f64,
}

#[component]
pub fn AddExpenseModal(mut props: AddExpenseModalProps) -> Element {
    let mut expense_name: Signal<String> = use_signal(|| "".to_string());
    let mut expense_description: Signal<Option<String>> = use_signal(|| None);
    let expense_type: Signal<ExpenseType> = use_signal(|| ExpenseType::Expense);
    let mut has_sent_form: Signal<bool> = use_signal(|| false);

    let checkbox_list: Vec<FormCheckboxAmount> = props
        .users
        .iter()
        .map(|user| FormCheckboxAmount {
            label: user.name.clone(),
            is_checked: false,
            amount: 0.0,
            user_id: user.id,
        })
        .collect();

    let mut expense_debtors: Signal<Vec<FormCheckboxAmount>> = use_signal(|| checkbox_list.clone());
    let mut expense_payers: Signal<Vec<FormCheckboxAmount>> = use_signal(|| checkbox_list.clone());

    let payers_total: Memo<f64> = use_memo(move || {
        expense_payers().iter().map(|e| e.amount).reduce(|acc, e| acc + e).unwrap_or(0.0)
    });
    let debtors_total = use_memo(move || {
        expense_debtors().iter().map(|d| d.amount).reduce(|acc, e| acc + e).unwrap_or(0.0)
    });
    let has_expense_amount_mismatch = use_memo(move || payers_total() != debtors_total());

    let handle_checkbox_change_debtors = move |data: (usize, bool)| {
        let (index, checked) = data;
        expense_debtors.with_mut(|items| {
            if let Some(item) = items.get_mut(index) {
                item.is_checked = checked;
                if !checked {
                    item.amount = 0.0;
                }
            }
        });
    };

    let handle_checkbox_change_payers = move |data: (usize, bool)| {
        let (index, checked) = data;
        expense_payers.with_mut(|items| {
            if let Some(item) = items.get_mut(index) {
                item.is_checked = checked;
                if !checked {
                    item.amount = 0.0;
                }
            }
        });
    };

    let handle_amount_change_debtors = move |data: (usize, f64)| {
        let (index, value) = data;
        expense_debtors.with_mut(|items| {
            if let Some(item) = items.get_mut(index) {
                item.amount = value;
            }
        });
    };

    let handle_amount_change_payers = move |data: (usize, f64)| {
        let (index, value) = data;
        expense_payers.with_mut(|items| {
            if let Some(item) = items.get_mut(index) {
                item.amount = value;
            }
        });
    };

    rsx! {
        dialog {
            id: "add_user_modal",
            class: "modal",
            class: if (props.is_expense_modal_open)() { "modal-open" } else { "" },
            div { class: "modal-box",
                h3 { class: "text-lg font-bold", "Ajouter un dépense" }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Nom de la dépense" }
                    input {
                        name: "expense_name",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| expense_name.set(event.value()),
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Description de la dépense" }
                    input {
                        name: "expense_description",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| expense_description.set(Some(event.value())),
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Qui a payé ? Total : {payers_total} €" }
                    for (index , item) in expense_payers().iter().enumerate() {
                        CheckboxFormItem {
                            index,
                            item: item.clone(),
                            on_checkbox_change: handle_checkbox_change_payers,
                            on_amount_change: handle_amount_change_payers,
                        }
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Qui doit rembourser ? Total : {debtors_total} €" }
                    for (index , item) in expense_debtors().iter().enumerate() {
                        CheckboxFormItem {
                            index,
                            item: item.clone(),
                            on_checkbox_change: handle_checkbox_change_debtors,
                            on_amount_change: handle_amount_change_debtors,
                        }
                    }
                }
                if has_expense_amount_mismatch() && has_sent_form() {
                    CalloutComponent {
                        callout_type: CalloutComponentTypes::error,
                        error_message: "payers_total : {payers_total()} € is different from debtors_total : {debtors_total()} €",
                    }
                }
                form {
                    method: "dialog",
                    onclick: move |_| props.is_expense_modal_open.set(false),
                    class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                    button { r#type: "button", "X" }
                }
                form { method: "dialog", class: "btn",
                    button {
                        r#type: "submit",
                        onclick: move |_| {
                            let users = props.users.clone();
                            spawn(async move {
                                has_sent_form.set(true);
                                let creatable_expense: CreatableExpense = CreatableExpense {
                                    name: expense_name(),
                                    description: expense_description(),
                                    amount: expense_payers()
                                        .iter()
                                        .map(|payer| payer.amount)
                                        .reduce(|acc, expense| acc + expense)
                                        .expect("ERROR while trying to compute expense amount sum"),
                                    expense_type: expense_type(),
                                    project_id: props.project_id,
                                    debtors: expense_debtors()
                                        .iter()
                                        .map(|debtor| UserAmount {
                                            amount: debtor.amount,
                                            user_id: debtor.user_id,
                                        })
                                        .collect(),
                                    payers: expense_payers()
                                        .iter()
                                        .map(|payer| UserAmount {
                                            amount: payer.amount,
                                            user_id: payer.user_id,
                                        })
                                        .collect(),
                                    author_id: users[0].id,
                                    date: chrono::NaiveDate::from_ymd_opt(1, 1, 1).unwrap(), // TODO
                                };
                                if has_expense_amount_mismatch() {
                                    info!(
                                        "payers_total : {:?} is different from total_debtors : {:?}",
                                        payers_total(), debtors_total()
                                    )
                                } else {
                                    props.is_expense_modal_open.set(false);
                                }
                            });
                        },
                        "Enregistrer"
                    }
                }
            }
            form {
                method: "dialog",
                class: "modal-backdrop",
                onclick: move |_| props.is_expense_modal_open.set(false),
                button { r#type: "button", "close" }
            }
        }
    }
}

#[component]
fn CheckboxFormItem(
    index: usize,
    item: FormCheckboxAmount,
    on_checkbox_change: EventHandler<(usize, bool)>,
    on_amount_change: EventHandler<(usize, f64)>,
) -> Element {
    rsx! {
        div { class: "flex items-center space-x-3 p-3 border rounded-lg justify-between",

            div { class: "space-x-3",
                // Checkbox
                input {
                    r#type: "checkbox",
                    class: "h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded",
                    checked: item.is_checked,
                    onchange: move |evt| {
                        on_checkbox_change.call((index, evt.checked()));
                    },
                }

                // Label
                label { class: "text-sm font-medium text-base-content", "{item.label}" }
            }

            // Input number
            input {
                r#type: "number",
                class: format!(
                    "px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 {}",
                    if item.is_checked { "" } else { "bg-gray-500 cursor-not-allowed" },
                ),
                disabled: !item.is_checked,
                value: item.amount,
                placeholder: if item.is_checked { "Entrez votre texte..." } else { "Cochez d'abord la case" },
                oninput: move |evt| {
                    if item.is_checked {
                        on_amount_change.call((index, evt.parsed().unwrap()));
                    }
                },
            }
        }
    }
}
