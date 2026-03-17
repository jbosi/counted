use api::expenses::expenses_controller::get_expense_by_id;
use api::payments::payments_controller::get_payments_by_expense_id;
use api::users::users_controller::get_users_by_project_id;
use dioxus::prelude::*;
use shared::ExpenseType;
use uuid::Uuid;

use crate::common::{initials, user_color_class, Avatar};
use crate::route::Route;

fn payers_title(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Gain     => "Contributeurs",
        ExpenseType::Transfer => "Émetteur",
        _                     => "Payé par",
    }
}

fn debtors_title(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Gain     => "Bénéficiaires",
        ExpenseType::Transfer => "Destinataires",
        _                     => "Débiteurs",
    }
}

#[component]
pub fn PaymentPage(project_id: Uuid, expense_id: i32) -> Element {
    let nav = use_navigator();

    let expense  = use_resource(move || async move { get_expense_by_id(expense_id).await });
    let payments = use_resource(move || async move { get_payments_by_expense_id(expense_id).await });
    let users    = use_resource(move || async move { get_users_by_project_id(project_id).await });

    rsx! {
        div { class: "container overflow-auto p-4 max-w-md w-full mx-auto flex flex-col gap-4",

            match (&*expense.read(), &*payments.read(), &*users.read()) {
                (Some(Ok(exp)), Some(Ok(pmts)), Some(Ok(user_list))) => {
                    let payers:  Vec<_> = pmts.iter().filter(|p| !p.is_debt).collect();
                    let debtors: Vec<_> = pmts.iter().filter(|p|  p.is_debt).collect();
                    let currency = project_id; // unused variable trick — we actually need the project
                    // Fetch currency from expense.project_id (same project_id from route)
                    let _ = currency;
                    let expense_type = exp.expense_type.clone();
                    let exp_name = exp.name.clone();
                    let exp_date = exp.date;

                    rsx! {
                        // Header
                        div { class: "navbar px-0",
                            div { class: "navbar-start",
                                button {
                                    r#type: "button",
                                    class: "btn btn-ghost btn-circle",
                                    onclick: move |_| { nav.push(Route::ProjectDetails { project_id }); },
                                    svg {
                                        class: "w-5 h-5",
                                        fill: "none",
                                        stroke: "currentColor",
                                        "stroke-width": "2",
                                        view_box: "0 0 24 24",
                                        path { d: "M15 18l-6-6 6-6" }
                                    }
                                }
                            }
                            div { class: "navbar-center flex flex-col items-center",
                                h1 { class: "text-base font-bold truncate max-w-[180px]", "{exp_name}" }
                                span { class: "text-xs text-base-content/60", "{exp_date.format(\"%d/%m/%Y\")}" }
                            }
                            div { class: "navbar-end",
                                details { class: "dropdown dropdown-end",
                                    summary { class: "btn btn-ghost btn-circle",
                                        svg {
                                            class: "w-5 h-5",
                                            fill: "none",
                                            stroke: "currentColor",
                                            "stroke-width": "2",
                                            view_box: "0 0 24 24",
                                            circle { cx: "12", cy: "12", r: "1" }
                                            circle { cx: "19", cy: "12", r: "1" }
                                            circle { cx: "5", cy: "12", r: "1" }
                                        }
                                    }
                                    ul { class: "menu dropdown-content bg-base-100 rounded-box w-40 shadow z-10 p-2",
                                        li { a { class: "btn-disabled opacity-50", "Modifier" } }
                                        li { a { class: "btn-disabled opacity-50 text-error", "Supprimer" } }
                                    }
                                }
                            }
                        }

                        // Payers section
                        div { class: "card bg-base-100 shadow-sm",
                            div { class: "card-body p-4 gap-3",
                                h2 { class: "font-semibold text-sm text-base-content/60 uppercase",
                                    "{payers_title(&expense_type)}"
                                }
                                for payment in payers {
                                    {
                                        let user = user_list.iter().find(|u| u.id == payment.user_id);
                                        let name = user.map(|u| u.name.as_str()).unwrap_or("?");
                                        let inits = initials(name);
                                        let color = user.map(|u| user_color_class(u.id)).unwrap_or("bg-neutral");
                                        let amount = payment.amount;
                                        rsx! {
                                            div { class: "flex items-center gap-3",
                                                Avatar { initials: inits, size: 9, color_class: color.to_string() }
                                                span { class: "flex-1 font-medium text-sm", "{name}" }
                                                span { class: "font-bold text-sm", "{amount:.2}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Debtors section
                        div { class: "card bg-base-100 shadow-sm",
                            div { class: "card-body p-4 gap-3",
                                h2 { class: "font-semibold text-sm text-base-content/60 uppercase",
                                    "{debtors_title(&expense_type)}"
                                }
                                for payment in debtors {
                                    {
                                        let user = user_list.iter().find(|u| u.id == payment.user_id);
                                        let name = user.map(|u| u.name.as_str()).unwrap_or("?");
                                        let inits = initials(name);
                                        let color = user.map(|u| user_color_class(u.id)).unwrap_or("bg-neutral");
                                        let amount = payment.amount;
                                        rsx! {
                                            div { class: "flex items-center gap-3",
                                                Avatar { initials: inits, size: 9, color_class: color.to_string() }
                                                span { class: "flex-1 font-medium text-sm", "{name}" }
                                                span { class: "font-bold text-sm", "{amount:.2}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                (Some(Err(e)), _, _) | (_, Some(Err(e)), _) | (_, _, Some(Err(e))) => rsx! {
                    div { class: "alert alert-error", "{e}" }
                },
                _ => rsx! {
                    div { class: "flex justify-center py-8",
                        span { class: "loading loading-spinner loading-md" }
                    }
                },
            }
        }
    }
}
