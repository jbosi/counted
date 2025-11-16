use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ToastProps {
    error: String,
    onclick: EventHandler<MouseEvent>,
}

#[component]
pub fn Toast(props: ToastProps) -> Element {
    rsx! {
        div {
            class: "toast toast-end",
            onclick: move |event| {
                event.stop_propagation();
                props.onclick.call(event)
            },
            div { class: "alert alert-error",
                span { class: "text-xs", "{props.error}" }
            }
        }
    }
}
