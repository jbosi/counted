use dioxus::prelude::*;
use ui::{Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "cards-container",
            div {
                class: "card",
                h3 {
                    class: "",
                    "Voyage en Allemagne"
                }
                p {
                    class: "",
                    "De la bière !"
                }
                button {
                    class: "button-action",
                    r#type: "button",
                    "En cours"
                }
            }
            div {
                class: "card",
                h3 {
                    class: "",
                    "Voyage en Italie"
                }
                p {
                    class: "",
                    "Petit voyage à 4 au soleil"
                }
                button {
                    class: "button-action",
                    r#type: "button",
                    "En cours"
                }
            }
        }
        Echo {}
    }
}
