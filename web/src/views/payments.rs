use crate::Route;
use dioxus::prelude::*;
use tracing::info;
use uuid::Uuid;
use api::{get_payments_by_expense_id, get_users_by_project_id};
use shared::{Expense, Payment, PaymentViewModel, User};
use ui::Avatar;

#[derive(PartialEq, Props, Clone)]
pub struct PaymentsProps {
    project_id: Uuid,
    expense_id: i32,
}

#[component]
pub fn Payments(props: PaymentsProps) -> Element {
    let mut payments: Signal<Vec<PaymentViewModel>> = use_signal(|| vec![]);
    let users_resource = use_resource(move || async move { get_users_by_project_id(props.project_id).await });

    use_resource({
        move || async move {
            match &*users_resource.value().read_unchecked() {
                Some(Ok(users)) => {
                    let user_ids: Vec<i32> = users.iter().map(|u| u.id).collect();
                    match get_payments_by_expense_id(props.expense_id).await {
                        Ok(response) => {
                            let result = response.into_iter().map(|p| {
                                let user: User = users.clone().into_iter().find(|u| u.id == p.user_id).unwrap();

                                PaymentViewModel {
                                    id: p.id,
                                    created_at: p.created_at,
                                    is_debt: p.is_debt,
                                    expense_id: p.expense_id,
                                    amount: p.amount,
                                    user,
                                }
                            }).collect();
                            payments.set(result);
                        }
                        Err(_) => ()
                    }
                }
                Some(Err(e)) => (),
                _ => {}
            }
    }});


    let debtors: Vec<PaymentViewModel> = payments()
        .into_iter()
        .filter(|p| p.is_debt)
        .collect();

    let payers: Vec<PaymentViewModel> = payments()
        .into_iter()
        .filter(|p| !p.is_debt)
        .collect();

    rsx! {
        section {
            class: "container flex flex-col max-w-md bg-base-100 p-4 rounded-xl",
            div {
                    class: "flex",
                    span {
                        "Répartition du payment"
                    }
                }
                PaymentList { payments: payers }
                div {
                    class: "flex ",
                    span {
                        "Répartition de la dette"
                    }
                }
                PaymentList { payments: debtors }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct PaymentListProps {
    payments: Vec<PaymentViewModel>,
}

#[component]
pub fn PaymentList(props: PaymentListProps) -> Element {
    rsx! {
        div {
            class: "container p-4 max-w-md rounded-xl flex flex-col",
            for payment in props.payments {
                div {
                    class: "flex items-center gap-4 p-3 hover:bg-base-200 rounded-lg transition-colors",
                    // Category
                    // Avatar { initials: "💰", size: 10 }

                    // Name
                    div {
                        class: "flex-1 min-w-0 flex-row flex items-center gap-3",
                        Avatar { initials: payment.user.name.get(0..2).unwrap_or("") },
                        p {
                            class: "font-semibold text-base-content truncate",
                            "{payment.user.name}"
                        }
                    }

                    // Amount
                    div {
                        class: "text-right",
                        p {
                            class: "font-bold text-lg text-base-content",
                            "{payment.amount} €"
                        }
                    }
                }
            }
        }
    }
}