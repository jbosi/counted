use crate::common::BackButtonArrow;
use crate::icons::BurgerIcon;
use crate::route::Route;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AppHeaderProps {
    title: String,
    sub_title: Option<String>,
    back_button_route: Route,
    children: Element,
}

#[component]
pub fn AppHeader(props: AppHeaderProps) -> Element {
    rsx! {
        div { class: "navbar px-0",
            div {
                class: "navbar-start",
                onclick: move |_| {
                    navigator().push(props.back_button_route.clone());
                },
                BackButtonArrow {}
            }
            div { class: "navbar-center",
                h1 { class: "text-xl font-bold", "{props.title}" }
                if let Some(sub_title) = props.sub_title {
                    div { class: "text-sm mb-3", "{sub_title}" }
                }

            }
            div { class: "navbar-end", {props.children} }
        }
    }
}
