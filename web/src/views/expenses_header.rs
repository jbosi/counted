use dioxus::prelude::*;
use ui::BackButtonArrow;
use crate::Route;

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesHeaderProps {
    title: String,
}

pub fn ExpensesHeader(props: ExpensesHeaderProps) -> Element {
    rsx! {
        div {
            class: "navbar px-0",
            div {
                class: "navbar-start",
                onclick: move |_| {
                    navigator().push(Route::Projects {});
                },
                BackButtonArrow {},
            }
            div {
                class: "navbar-center",
                h1 { class: "text-xl font-bold", "{props.title}" }
            }
            div {
                class: "navbar-end",
                button {
                    class: "btn btn-ghost btn-circle",
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        view_box: "0 0 24 24",
                        path { d: "M3 12h18M3 6h18M3 18h18" }
                    },
                }
            }
        }
    }
}