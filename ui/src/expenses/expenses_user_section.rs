use crate::common::Avatar;
use crate::modals::AddUserModal;
use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use shared::User;
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesUserSectionProps {
    id: Uuid,
    users: Vec<User>,
}

#[component]
pub fn ExpensesUserSection(props: ExpensesUserSectionProps) -> Element {
    let mut is_user_modal_open = use_signal(|| false);
    rsx! {
        div { class: "flex p-4 justify-center",

            div { class: "avatar-group -space-x-4",
                for user in props.users {
                    Avatar { initials: user.name.get(0..2).unwrap_or(""), size: 12 }
                }
                button {
                    type: "button",
                    class: "btn btn-circle btn-outline btn-lg self-center",
                    onclick: move |_| is_user_modal_open.set(true),
                    "+"
                }
            }
        }
        AddUserModal { is_user_modal_open, id: props.id }
    }
}
