use crate::common::Avatar;
use crate::route::Route;
use dioxus::prelude::*;
use shared::{Expense, User};

#[derive(PartialEq, Props, Clone)]
pub struct ExpenseBarChartProps {
    user: User,
    summary_amount: f64,
    max_amount: f64,
}

#[component]
pub fn ExpenseBarChartComponent(props: ExpenseBarChartProps) -> Element {
    let summary_amount: f64 = props.summary_amount;
    let max_amount: f64 = props.max_amount;

    rsx! {
        div { class: "flex gap-2 justify-between",
            div { class: "flex gap-2",
                Avatar {
                    initials: props.user.name.get(0..2).unwrap_or(""),
                    size: 12,
                }
                div { class: "self-center",
                    span {
                        if summary_amount.is_sign_negative() {
                            "{summary_amount} €"
                        } else {
                            "+{summary_amount} €"
                        }
                    }
                }
            }
            progress {
                class: if summary_amount > 0.0 { "progress progress-primary self-center" } else { "progress progress-error self-center" },
                style: if summary_amount < 0.0 { "transform: translateX(-100%);" },
                style: "width: {(summary_amount.abs() * 30.0) / max_amount}%",
                value: "100",
                max: "100",
            }
        }
    }
}
