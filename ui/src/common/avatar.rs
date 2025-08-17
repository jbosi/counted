use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AvatarProps {
    initials: String,
    size: Option<u8>,
    text_size: Option<String>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let size = props.size.unwrap_or_else(|| 8);
    let text_size = props.text_size.unwrap_or_else(|| "xs".to_string());

    rsx! {
        div { class: "avatar avatar-placeholder",
            div { class: "bg-primary-content w-{size} rounded-full",
                span { class: "text-{text_size} text-base-100", "{props.initials}" }
            }
        }
    }
}
