use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SummaryCardProps {
    my_total: f64,
    global_total: f64,
}

#[component]
pub fn SummaryCard(props: SummaryCardProps) -> Element {
    let format_currency = |val: f64| format!("{:.2}€", val).replace('.', ",");

    rsx! {
        div { class: "",
            div { class: "card-body p-4 space-y-3",

                div { class: "flex justify-between items-center",
                    span { class: "text-base-content", "Mon Total" }
                    span { class: "font-bold text-lg", "{format_currency(props.my_total)}" }
                }

                div { class: "flex justify-between items-center",
                    span { class: "text-base-content/70", "Total global" }
                    span { class: "font-semibold text-lg text-base-content/70",
                        "{format_currency(props.global_total)}"
                    }
                }
            }
        }
    }
}
