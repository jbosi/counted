use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ToastProps {
    error: String,
}

#[component]
pub fn Toast(props: ToastProps) -> Element {
    rsx! {
        div { class: "toast toast-end",
            div { class: "alert alert-error",
                span { class: "text-xs", "{props.error}" }
            }
        }
    }
}
