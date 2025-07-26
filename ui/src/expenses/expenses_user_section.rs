use crate::{AddUserModal, Avatar};
use dioxus::hooks::use_signal;
use dioxus::prelude::*;
use shared::User;
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesUserSectionProps {
    id: Uuid,
    users: Vec<User>,
}

pub fn ExpensesUserSection(props: ExpensesUserSectionProps) -> Element {
    let mut is_user_modal_open = use_signal(|| false);
    rsx! {
        div {
            class: "flex justify-between items-center my-6 p-4",

            button {
                class: "btn btn-circle btn-outline btn-lg",
                onclick: move |_| is_user_modal_open.set(true),
                "+"
            }

            div {
                class: "avatar-group -space-x-4",
                for user in props.users {
                    Avatar { initials: user.name.get(0..2).unwrap_or(""), size: 12 }
                }
            }

            div { class: "w-16" }
        }
        AddUserModal { is_user_modal_open, id: props.id }
    }
}