use dioxus::prelude::*;
use shared::{User, UserSummary};

use crate::common::{initials, user_color_class, Avatar};

#[derive(PartialEq, Props, Clone)]
pub struct BalanceTabProps {
    pub summary: UserSummary,
    pub users: Vec<User>,
    pub currency: String,
}

#[component]
pub fn BalanceTab(props: BalanceTabProps) -> Element {
    let mut entries: Vec<(User, f64)> = props
        .users
        .iter()
        .map(|u| {
            let balance = props.summary.summary.get(&u.id).copied().unwrap_or(0.0);
            (u.clone(), balance)
        })
        .collect();

    // Sort ascending (most negative first)
    entries.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let max_abs = entries
        .iter()
        .map(|(_, b)| b.abs())
        .fold(1.0_f64, f64::max);

    let currency = props.currency.clone();

    rsx! {
        ul { class: "flex flex-col gap-2",
            for (user, balance) in entries {
                {
                    let width = ((balance.abs() / max_abs) * 100.0).max(2.0);
                    let (amount_class, bar_class) = if balance > 0.0 {
                        ("text-success font-semibold", "progress-success")
                    } else if balance < 0.0 {
                        ("text-error font-semibold", "progress-error")
                    } else {
                        ("text-base-content/60", "")
                    };
                    let sign = if balance > 0.0 { "+" } else { "" };
                    let curr = currency.clone();
                    rsx! {
                        li { class: "bg-base-100 rounded-lg shadow-sm p-3",
                            div { class: "flex items-center gap-2 mb-2",
                                Avatar {
                                    initials: initials(&user.name),
                                    size: 8,
                                    color_class: user_color_class(user.id).to_string(),
                                }
                                span { class: "flex-1 font-medium text-sm truncate", "{user.name}" }
                                span { class: "text-sm {amount_class}",
                                    "{sign}{balance:.2} {curr}"
                                }
                            }
                            if balance != 0.0 {
                                div { class: "flex items-center gap-1",
                                    if balance < 0.0 {
                                        // negative: bar on left side
                                        progress {
                                            class: "progress {bar_class} flex-none",
                                            style: "width: {width:.0}%",
                                            value: "100",
                                            max: "100",
                                        }
                                        div { class: "flex-1" }
                                    } else {
                                        // positive: bar on right side
                                        div { class: "flex-1" }
                                        progress {
                                            class: "progress {bar_class} flex-none",
                                            style: "width: {width:.0}%",
                                            value: "100",
                                            max: "100",
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
