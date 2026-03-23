use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AvatarProps {
    initials: String,
    size: Option<u8>,
    color_class: Option<String>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let size = props.size.unwrap_or(8);
    let color = props.color_class.as_deref().unwrap_or("bg-primary-content");

    rsx! {
        div { class: "avatar avatar-placeholder",
            div { class: "{color} w-{size} rounded-full",
                span { class: "text-xs text-base-100", "{props.initials}" }
            }
        }
    }
}
