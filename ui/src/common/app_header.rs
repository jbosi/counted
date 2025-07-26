use crate::common::BackButtonArrow;
use crate::route::Route;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AppHeaderProps {
    title: String,
    back_button_route: Route
}

pub fn AppHeader(props: AppHeaderProps) -> Element {
    rsx! {
        div {
            class: "navbar px-0",
            div {
                class: "navbar-start",
                onclick: move |_| {
                    navigator().push(props.back_button_route.clone());
                },
                BackButtonArrow {},
            }
            div {
                class: "navbar-center",
                h1 {
                    class: "text-xl font-bold", "{props.title}"
                }
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