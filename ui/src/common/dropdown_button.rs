use dioxus::document::Eval;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct DropdownButtonProps {
    first_component: Element,
    second_component: Element,
}
#[component]
pub fn DropdownButton(mut props: DropdownButtonProps) -> Element {
    rsx! {
        details {
            class: "dropdown dropdown-left",
            onclick: move |event| {
                event.stop_propagation();
            },
            summary {
                class: "btn btn-ghost btn-circle",
                "..."
            },
            ul {
                class: "menu dropdown-content rounded-box z-1 w-52 p-2 shadow-sm",
                popover: "",
                id: "popover-project-dot",
                style: "position-anchor:--anchor-project-dot",
                li {
                    {props.first_component}
                }
                li {
                    {props.second_component}
                }
            }
        }
    }
}

fn close_dropdown() -> Eval {
    document::eval(
        "document.activeElement.closest('.dropdown').removeAttribute('open'); return null",
    )
}
