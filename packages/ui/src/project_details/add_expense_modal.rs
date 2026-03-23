use api::expenses::expenses_controller::add_expense;
use chrono::NaiveDate;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{CreatableExpense, ExpenseType, User, UserAmount};
use uuid::Uuid;

#[derive(Clone, PartialEq)]
struct UserEntry {
    user: User,
    checked: bool,
    amount: f64,
    shares: u32,
}

fn distribute(total: f64, entries: &mut Vec<UserEntry>) {
    let checked_indices: Vec<usize> = entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.checked)
        .map(|(i, _)| i)
        .collect();
    let n = checked_indices.len();
    if n == 0 {
        return;
    }
    let total_cents = (total * 100.0).round() as i64;
    let base_cents = total_cents / n as i64;
    let remainder = (total_cents % n as i64) as usize;
    for (pos, &idx) in checked_indices.iter().enumerate() {
        let cents = if pos < remainder { base_cents + 1 } else { base_cents };
        entries[idx].amount = cents as f64 / 100.0;
    }
    for entry in entries.iter_mut() {
        if !entry.checked {
            entry.amount = 0.0;
        }
    }
}

fn distribute_by_shares(total: f64, entries: &mut Vec<UserEntry>) {
    let total_shares: u32 = entries
        .iter()
        .filter(|e| e.checked && e.shares > 0)
        .map(|e| e.shares)
        .sum();
    if total_shares == 0 {
        return;
    }
    let total_cents = (total * 100.0).round() as i64;
    let checked_indices: Vec<usize> = entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.checked && e.shares > 0)
        .map(|(i, _)| i)
        .collect();
    let mut remaining = total_cents;
    for (pos, &idx) in checked_indices.iter().enumerate() {
        let alloc = if pos == checked_indices.len() - 1 {
            remaining
        } else {
            (entries[idx].shares as i64 * total_cents) / total_shares as i64
        };
        entries[idx].amount = alloc as f64 / 100.0;
        remaining -= alloc;
    }
    for entry in entries.iter_mut() {
        if !entry.checked {
            entry.amount = 0.0;
        }
    }
}

fn payers_label(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Gain => "Qui a reçu ?",
        ExpenseType::Transfer => "Qui transfère ?",
        _ => "Qui a payé ?",
    }
}

fn debtors_label(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Transfer => "Qui reçoit ?",
        ExpenseType::Gain => "Qui a contribué ?",
        _ => "Qui doit ?",
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AddExpenseModalProps {
    pub on_close: EventHandler<()>,
    pub on_created: EventHandler<()>,
    pub project_id: Uuid,
    pub users: Vec<User>,
    pub stored_user_id: Option<i32>,
    // Optional pre-fill (used when opening from a reimbursement suggestion)
    pub initial_name: Option<String>,
    pub initial_amount: Option<f64>,
    pub initial_expense_type: Option<ExpenseType>,
    pub initial_payer_id: Option<i32>,
    pub initial_debtor_id: Option<i32>,
}

#[component]
pub fn AddExpenseModal(props: AddExpenseModalProps) -> Element {
    let init_name = props.initial_name.clone().unwrap_or_default();
    let init_amount = props.initial_amount.unwrap_or(0.0);
    let init_type = props.initial_expense_type.clone().unwrap_or(ExpenseType::Expense);
    let initial_payer_id = props.initial_payer_id;
    let initial_debtor_id = props.initial_debtor_id;

    let mut expense_name = use_signal(move || init_name);
    let today = chrono::Utc::now()
        .naive_utc()
        .date()
        .format("%Y-%m-%d")
        .to_string();
    let mut date_str = use_signal(move || today);
    let mut total_amount = use_signal(move || init_amount);
    let mut expense_type = use_signal(move || init_type);
    let mut payers_share_mode = use_signal(|| false);
    let mut debtors_share_mode = use_signal(|| false);

    let init_payers = props.users.clone();
    let mut payers: Signal<Vec<UserEntry>> = use_signal(move || {
        init_payers
            .iter()
            .map(|u| {
                let is_pre = initial_payer_id == Some(u.id);
                UserEntry {
                    user: u.clone(),
                    checked: is_pre,
                    amount: if is_pre { init_amount } else { 0.0 },
                    shares: if is_pre { 1 } else { 0 },
                }
            })
            .collect()
    });
    let init_debtors = props.users.clone();
    let mut debtors: Signal<Vec<UserEntry>> = use_signal(move || {
        init_debtors
            .iter()
            .map(|u| {
                if let Some(did) = initial_debtor_id {
                    let is_pre = u.id == did;
                    UserEntry {
                        user: u.clone(),
                        checked: is_pre,
                        amount: if is_pre { init_amount } else { 0.0 },
                        shares: if is_pre { 1 } else { 0 },
                    }
                } else {
                    UserEntry { user: u.clone(), checked: true, amount: 0.0, shares: 1 }
                }
            })
            .collect()
    });
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);

    let project_id = props.project_id;
    let stored_user_id = props.stored_user_id;
    let users_for_author = props.users.clone();
    let on_created = props.on_created.clone();
    let on_close_submit = props.on_close.clone();

    let on_submit = move |e: FormEvent| {
        e.prevent_default();

        let name_val = expense_name().trim().to_string();
        if name_val.is_empty() {
            error_msg.set(Some("Le nom est requis.".into()));
            return;
        }
        let total = total_amount();
        if total <= 0.0 {
            error_msg.set(Some("Le montant doit être supérieur à 0.".into()));
            return;
        }
        let active_payers: Vec<UserAmount> = payers()
            .iter()
            .filter(|e| e.checked && e.amount > 0.0)
            .map(|e| UserAmount { user_id: e.user.id, amount: e.amount })
            .collect();
        let active_debtors: Vec<UserAmount> = debtors()
            .iter()
            .filter(|e| e.checked && e.amount > 0.0)
            .map(|e| UserAmount { user_id: e.user.id, amount: e.amount })
            .collect();
        if active_payers.is_empty() {
            error_msg.set(Some("Sélectionnez au moins un payeur.".into()));
            return;
        }
        if active_debtors.is_empty() {
            error_msg.set(Some("Sélectionnez au moins un débiteur.".into()));
            return;
        }
        let date = match NaiveDate::parse_from_str(&date_str(), "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => {
                error_msg.set(Some("Date invalide.".into()));
                return;
            }
        };
        let author_id = stored_user_id
            .unwrap_or_else(|| users_for_author.first().map(|u| u.id).unwrap_or(0));
        let etype = expense_type();

        loading.set(true);
        error_msg.set(None);

        let on_created = on_created.clone();
        let on_close_submit = on_close_submit.clone();
        spawn(async move {
            match add_expense(Json(CreatableExpense {
                name: name_val,
                amount: total,
                expense_type: etype,
                project_id,
                payers: active_payers,
                debtors: active_debtors,
                author_id,
                description: None,
                date,
            }))
            .await
            {
                Ok(_) => {
                    on_created.call(());
                    on_close_submit.call(());
                }
                Err(e) => {
                    error_msg.set(Some(e.to_string()));
                    loading.set(false);
                }
            }
        });
    };

    let on_close_x = props.on_close.clone();
    let on_close_cancel = props.on_close.clone();
    let on_close_backdrop = props.on_close.clone();

    rsx! {
        div { class: "modal modal-open", role: "dialog",
            div { class: "modal-box max-w-md relative",
                button {
                    r#type: "button",
                    class: "btn btn-ghost btn-sm btn-circle absolute right-2 top-2",
                    onclick: move |_| on_close_x.call(()),
                    "✕"
                }

                h3 { class: "font-bold text-lg mb-4", "Ajouter une dépense" }

                if let Some(err) = error_msg() {
                    div { class: "alert alert-error text-sm mb-3", "{err}" }
                }

                form {
                    class: "flex flex-col gap-4",
                    onsubmit: on_submit,

                    label { class: "form-control",
                        span { class: "label-text mb-1", "Nom *" }
                        input {
                            class: "input input-bordered",
                            r#type: "text",
                            value: "{expense_name}",
                            oninput: move |e| expense_name.set(e.value()),
                        }
                    }

                    label { class: "form-control",
                        span { class: "label-text mb-1", "Date" }
                        input {
                            class: "input input-bordered",
                            r#type: "date",
                            value: "{date_str}",
                            oninput: move |e| date_str.set(e.value()),
                        }
                    }

                    label { class: "form-control",
                        span { class: "label-text mb-1", "Montant *" }
                        input {
                            class: "input input-bordered",
                            r#type: "number",
                            step: "0.01",
                            min: "0",
                            value: "{total_amount}",
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<f64>() {
                                    total_amount.set(v);
                                }
                            },
                            onblur: move |_| {
                                let t = total_amount();
                                if payers_share_mode() {
                                    distribute_by_shares(t, &mut payers.write());
                                } else {
                                    distribute(t, &mut payers.write());
                                }
                                if debtors_share_mode() {
                                    distribute_by_shares(t, &mut debtors.write());
                                } else {
                                    distribute(t, &mut debtors.write());
                                }
                            },
                        }
                    }

                    label { class: "form-control",
                        span { class: "label-text mb-1", "Type" }
                        select {
                            class: "select select-bordered",
                            value: match expense_type() {
                                ExpenseType::Expense => "Expense",
                                ExpenseType::Gain => "Gain",
                                ExpenseType::Transfer => "Transfer",
                            },
                            oninput: move |e| {
                                let t = match e.value().as_str() {
                                    "Gain" => ExpenseType::Gain,
                                    "Transfer" => ExpenseType::Transfer,
                                    _ => ExpenseType::Expense,
                                };
                                expense_type.set(t);
                            },
                            option { value: "Expense", "Dépense" }
                            option { value: "Gain", "Gain" }
                            option { value: "Transfer", "Transfert" }
                        }
                    }

                    // Payers fieldset
                    fieldset { class: "border rounded-box border-base-300 p-3",
                        legend { class: "px-2 text-sm font-medium",
                            "{payers_label(&expense_type())}"
                        }
                        div { class: "flex items-center justify-between mb-2",
                            button {
                                r#type: "button",
                                class: "btn btn-ghost btn-xs",
                                onclick: move |_| {
                                    let t = total_amount();
                                    let all_checked = payers().iter().all(|e| e.checked);
                                    let new_checked = !all_checked;
                                    let sm = payers_share_mode();
                                    let mut p = payers.write();
                                    for entry in p.iter_mut() {
                                        entry.checked = new_checked;
                                        if !new_checked {
                                            entry.amount = 0.0;
                                            entry.shares = 0;
                                        } else if sm && entry.shares == 0 {
                                            entry.shares = 1;
                                        }
                                    }
                                    if new_checked {
                                        if sm {
                                            distribute_by_shares(t, &mut *p);
                                        } else {
                                            distribute(t, &mut *p);
                                        }
                                    }
                                },
                                if payers().iter().all(|e| e.checked) { "Tout désélectionner" } else { "Tout sélectionner" }
                            }
                            label { class: "flex items-center gap-2 cursor-pointer",
                                span { class: "text-xs", "Par parts" }
                                input {
                                    r#type: "checkbox",
                                    class: "toggle toggle-sm",
                                    checked: payers_share_mode(),
                                    oninput: move |_| {
                                        let t = total_amount();
                                        let new_mode = !payers_share_mode();
                                        payers_share_mode.set(new_mode);
                                        let mut p = payers.write();
                                        if new_mode {
                                            for entry in p.iter_mut() {
                                                if entry.checked && entry.shares == 0 {
                                                    entry.shares = 1;
                                                }
                                            }
                                            distribute_by_shares(t, &mut *p);
                                        } else {
                                            distribute(t, &mut *p);
                                        }
                                    },
                                }
                            }
                        }
                        div { class: "flex flex-col gap-2",
                            for i in 0..payers().len() {
                                {
                                    let name = payers().get(i).map(|e| e.user.name.clone()).unwrap_or_default();
                                    let checked = payers().get(i).map(|e| e.checked).unwrap_or(false);
                                    let amount_val = payers().get(i).map(|e| e.amount).unwrap_or(0.0);
                                    let shares_val = payers().get(i).map(|e| e.shares).unwrap_or(0);
                                    let sm = payers_share_mode();
                                    rsx! {
                                        div { class: "flex items-center gap-3",
                                            input {
                                                r#type: "checkbox",
                                                class: "checkbox checkbox-sm",
                                                checked: checked,
                                                oninput: move |_| {
                                                    let t = total_amount();
                                                    let sm = payers_share_mode();
                                                    let mut p = payers.write();
                                                    if let Some(entry) = p.get_mut(i) {
                                                        entry.checked = !entry.checked;
                                                        if sm {
                                                            if entry.checked && entry.shares == 0 {
                                                                entry.shares = 1;
                                                            } else if !entry.checked {
                                                                entry.shares = 0;
                                                            }
                                                        }
                                                    }
                                                    if sm {
                                                        distribute_by_shares(t, &mut *p);
                                                    } else {
                                                        distribute(t, &mut *p);
                                                    }
                                                },
                                            }
                                            span { class: "flex-1 text-sm", "{name}" }
                                            if sm {
                                                span { class: "text-xs text-base-content/60 w-16 text-right", "{amount_val:.2}" }
                                                input {
                                                    class: "input input-sm w-16",
                                                    r#type: "number",
                                                    step: "1",
                                                    min: "0",
                                                    value: "{shares_val}",
                                                    oninput: move |e| {
                                                        let v = e.value().parse::<u32>().unwrap_or(0);
                                                        let t = total_amount();
                                                        let mut p = payers.write();
                                                        if let Some(entry) = p.get_mut(i) {
                                                            entry.shares = v;
                                                            entry.checked = v > 0;
                                                        }
                                                        distribute_by_shares(t, &mut *p);
                                                    },
                                                }
                                            } else {
                                                input {
                                                    class: "input input-sm w-28",
                                                    r#type: "number",
                                                    step: "0.01",
                                                    min: "0",
                                                    value: "{amount_val}",
                                                    oninput: move |e| {
                                                        if let Ok(v) = e.value().parse::<f64>() {
                                                            let mut p = payers.write();
                                                            if let Some(entry) = p.get_mut(i) {
                                                                entry.amount = v;
                                                            }
                                                        }
                                                    },
                                                    onblur: move |_| {
                                                        let mut p = payers.write();
                                                        if let Some(entry) = p.get_mut(i) {
                                                            if entry.amount > 0.0 && !entry.checked {
                                                                entry.checked = true;
                                                            } else if entry.amount == 0.0 && entry.checked {
                                                                entry.checked = false;
                                                            }
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Debtors fieldset
                    fieldset { class: "border rounded-box border-base-300 p-3",
                        legend { class: "px-2 text-sm font-medium",
                            "{debtors_label(&expense_type())}"
                        }
                        div { class: "flex items-center justify-between mb-2",
                            button {
                                r#type: "button",
                                class: "btn btn-ghost btn-xs",
                                onclick: move |_| {
                                    let t = total_amount();
                                    let all_checked = debtors().iter().all(|e| e.checked);
                                    let new_checked = !all_checked;
                                    let sm = debtors_share_mode();
                                    let mut d = debtors.write();
                                    for entry in d.iter_mut() {
                                        entry.checked = new_checked;
                                        if !new_checked {
                                            entry.amount = 0.0;
                                            entry.shares = 0;
                                        } else if sm && entry.shares == 0 {
                                            entry.shares = 1;
                                        }
                                    }
                                    if new_checked {
                                        if sm {
                                            distribute_by_shares(t, &mut *d);
                                        } else {
                                            distribute(t, &mut *d);
                                        }
                                    }
                                },
                                if debtors().iter().all(|e| e.checked) { "Tout désélectionner" } else { "Tout sélectionner" }
                            }
                            label { class: "flex items-center gap-2 cursor-pointer",
                                span { class: "text-xs", "Par parts" }
                                input {
                                    r#type: "checkbox",
                                    class: "toggle toggle-sm",
                                    checked: debtors_share_mode(),
                                    oninput: move |_| {
                                        let t = total_amount();
                                        let new_mode = !debtors_share_mode();
                                        debtors_share_mode.set(new_mode);
                                        let mut d = debtors.write();
                                        if new_mode {
                                            for entry in d.iter_mut() {
                                                if entry.checked && entry.shares == 0 {
                                                    entry.shares = 1;
                                                }
                                            }
                                            distribute_by_shares(t, &mut *d);
                                        } else {
                                            distribute(t, &mut *d);
                                        }
                                    },
                                }
                            }
                        }
                        div { class: "flex flex-col gap-2",
                            for i in 0..debtors().len() {
                                {
                                    let name = debtors().get(i).map(|e| e.user.name.clone()).unwrap_or_default();
                                    let checked = debtors().get(i).map(|e| e.checked).unwrap_or(false);
                                    let amount_val = debtors().get(i).map(|e| e.amount).unwrap_or(0.0);
                                    let shares_val = debtors().get(i).map(|e| e.shares).unwrap_or(0);
                                    let sm = debtors_share_mode();
                                    rsx! {
                                        div { class: "flex items-center gap-3",
                                            input {
                                                r#type: "checkbox",
                                                class: "checkbox checkbox-sm",
                                                checked: checked,
                                                oninput: move |_| {
                                                    let t = total_amount();
                                                    let sm = debtors_share_mode();
                                                    let mut d = debtors.write();
                                                    if let Some(entry) = d.get_mut(i) {
                                                        entry.checked = !entry.checked;
                                                        if sm {
                                                            if entry.checked && entry.shares == 0 {
                                                                entry.shares = 1;
                                                            } else if !entry.checked {
                                                                entry.shares = 0;
                                                            }
                                                        }
                                                    }
                                                    if sm {
                                                        distribute_by_shares(t, &mut *d);
                                                    } else {
                                                        distribute(t, &mut *d);
                                                    }
                                                },
                                            }
                                            span { class: "flex-1 text-sm", "{name}" }
                                            if sm {
                                                span { class: "text-xs text-base-content/60 w-16 text-right", "{amount_val:.2}" }
                                                input {
                                                    class: "input input-sm w-16",
                                                    r#type: "number",
                                                    step: "1",
                                                    min: "0",
                                                    value: "{shares_val}",
                                                    oninput: move |e| {
                                                        let v = e.value().parse::<u32>().unwrap_or(0);
                                                        let t = total_amount();
                                                        let mut d = debtors.write();
                                                        if let Some(entry) = d.get_mut(i) {
                                                            entry.shares = v;
                                                            entry.checked = v > 0;
                                                        }
                                                        distribute_by_shares(t, &mut *d);
                                                    },
                                                }
                                            } else {
                                                input {
                                                    class: "input input-sm w-28",
                                                    r#type: "number",
                                                    step: "0.01",
                                                    min: "0",
                                                    value: "{amount_val}",
                                                    oninput: move |e| {
                                                        if let Ok(v) = e.value().parse::<f64>() {
                                                            let mut d = debtors.write();
                                                            if let Some(entry) = d.get_mut(i) {
                                                                entry.amount = v;
                                                            }
                                                        }
                                                    },
                                                    onblur: move |_| {
                                                        let mut d = debtors.write();
                                                        if let Some(entry) = d.get_mut(i) {
                                                            if entry.amount > 0.0 && !entry.checked {
                                                                entry.checked = true;
                                                            } else if entry.amount == 0.0 && entry.checked {
                                                                entry.checked = false;
                                                            }
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "modal-action mt-2",
                        button {
                            r#type: "button",
                            class: "btn",
                            onclick: move |_| on_close_cancel.call(()),
                            "Annuler"
                        }
                        button {
                            r#type: "submit",
                            class: "btn btn-primary",
                            disabled: loading(),
                            if loading() { "Ajout…" } else { "Ajouter" }
                        }
                    }
                }
            }

            div {
                class: "modal-backdrop",
                onclick: move |_| on_close_backdrop.call(()),
            }
        }
    }
}
