use dioxus::prelude::*;

use crate::icons::BackArrowIcon;

#[component]
pub fn BackButtonArrow() -> Element {
    rsx! {
        button { r#type: "button", class: "btn btn-circle", BackArrowIcon {} }
    }
}
