use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AvatarProps {
    initials: String,
    size: Option<u8>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let size = props.size.unwrap_or_else(|| 8);

    rsx! {
        div { class: "avatar avatar-placeholder",
            div { class: "bg-primary-content w-{size} rounded-full",
                span { class: "text-xs text-base-100", "{props.initials}" }
            }
        }
    }
}
