use crate::common::{Avatar, BackButtonArrow, DropdownButton};
use crate::route::Route;
use crate::utils::close_dropdown;
use api::expenses::{delete_expense_by_id, get_expense_by_id};
use api::payments::get_payments_by_expense_id;
use api::users::get_users_by_project_id;
use dioxus::prelude::*;
use shared::api::{ApiError, ApiState};
use shared::{PaymentViewModel, User};
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct PaymentsProps {
    project_id: Uuid,
    expense_id: i32,
}

#[component]
pub fn Payments(props: PaymentsProps) -> Element {
    let mut payments: Signal<Vec<PaymentViewModel>> = use_signal(|| vec![]);
    let users_resource =
        use_resource(move || async move { get_users_by_project_id(props.project_id).await });
    let expense_resource =
        use_resource(move || async move { get_expense_by_id(props.expense_id).await });

    let _ = use_resource({
        move || async move {
            match &*users_resource.value().read_unchecked() {
                Some(Ok(users)) => match get_payments_by_expense_id(props.expense_id).await {
                    Ok(response) => {
                        let result = response
                            .into_iter()
                            .map(|p| {
                                let user: User =
                                    users.clone().into_iter().find(|u| u.id == p.user_id).unwrap();

                                PaymentViewModel {
                                    id: p.id,
                                    created_at: p.created_at,
                                    is_debt: p.is_debt,
                                    expense_id: p.expense_id,
                                    amount: p.amount,
                                    user,
                                }
                            })
                            .collect();
                        payments.set(result);
                    }
                    Err(_) => (),
                },
                Some(Err(_e)) => (),
                _ => {}
            }
        }
    });

    let debtors: Vec<PaymentViewModel> = payments().into_iter().filter(|p| p.is_debt).collect();

    let payers: Vec<PaymentViewModel> = payments().into_iter().filter(|p| !p.is_debt).collect();

    let total_payment: f64 =
        payers.clone().into_iter().map(|p| p.amount).reduce(|acc, e| acc + e).unwrap_or(0.0);

    let mut api_expense_delete_state = use_signal(|| ApiState::<()>::Loading);

    rsx! {
        div {
            class: "container overflow-auto app-container bg-base-200 p-4 max-w-md rounded-xl flex flex-col gap-6",
            section {
                class: "flex flex-col gap-3",
                if let Some(expense) = &*expense_resource.read() {
                    match expense {
                        Ok(e) => rsx! {
                            div { class: "flex flex-row",
                                div {
                                    class: "navbar-start flex-1",
                                    onclick: move |_| {
                                        navigator()
                                            .push(Route::Expenses {
                                                project_id: props.project_id,
                                            });
                                    },
                                    BackButtonArrow {}
                                }
                                h1 { class: "text-xl font-bold self-center flex-grow", "{e.name}" }
                                DropdownButton {
                            first_component: rsx! {
                                button {
                                    class: "btn btn-ghost",
                                    onclick: move |event| async move {
                                        close_dropdown().await.unwrap_or("".into());

                                        // update_expense_modal_open.set(true);
                                    },
                                    "Editer"
                                }
                            },
                            second_component: rsx! {
                                button {
                                    class: "btn btn-ghost",
                                    onclick: move |_| {
                                        spawn(async move {
                                            close_dropdown().await.unwrap_or("".into());

                                            match delete_expense_by_id(props.expense_id).await {
                                                Ok(()) => api_expense_delete_state.set(ApiState::Success(())),
                                                Err(error) => api_expense_delete_state.set(ApiState::Error(ApiError(error.to_string())))
                                            };
                                        });
                                    },
                                    "Supprimer"
                                }
                            }
                        },
                            }
                            span { class: "self-center", "Dépense de {total_payment} €" }
                            match e.clone().description {
                                Some(description) => rsx! {
                                    span { "{description}" }
                                },
                                None => rsx! { "" },
                            }
                        },
                        Err(err) => rsx! {
                        "{err}"
                        },
                    }
                }
            }
            section {
                class: "flex flex-col",
                div { class: "flex",
                    span { "Répartition du paiement" }
                }
                PaymentList { payments: payers, is_debt: false }
                div { class: "flex ",
                    span { "Répartition de la dette" }
                }
                PaymentList { payments: debtors, is_debt: true }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct PaymentListProps {
    payments: Vec<PaymentViewModel>,
    is_debt: bool,
}

#[component]
pub fn PaymentList(props: PaymentListProps) -> Element {
    rsx! {
        div { class: "container p-4 max-w-md rounded-xl flex flex-col",
            for payment in props.payments {
                div { class: "flex items-center gap-4 p-3 hover:bg-base-200 rounded-lg transition-colors",
                    // Category
                    // Avatar { initials: "💰", size: 10 }

                    // Name
                    div { class: "flex-1 min-w-0 flex-row flex items-center gap-3",
                        Avatar { initials: payment.user.name.get(0..2).unwrap_or("") }
                        p { class: "font-semibold text-base-content truncate",
                            "{payment.user.name}"
                            if props.is_debt {
                                " doit"
                            } else {
                                " a payé"
                            }
                        }
                    }

                    // Amount
                    div { class: "text-right",
                        p { class: "font-bold text-lg text-base-content", "{payment.amount} €" }
                    }
                }
            }
        }
    }
}
