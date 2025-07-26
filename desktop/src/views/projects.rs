use dioxus::prelude::*;
use ui::{Echo, Hero};

#[component]
pub fn Projects() -> Element {
    rsx! {
        Hero {}
        Echo {}
    }
}
