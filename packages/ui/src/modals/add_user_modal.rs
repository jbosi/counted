use api::users::add_user;
use dioxus::prelude::*;
use shared::CreatableUser;
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct AddUserModalProps {
    id: Uuid,
    is_user_modal_open: Signal<bool>,
}
#[component]
pub fn AddUserModal(mut props: AddUserModalProps) -> Element {
    let mut user_name: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        dialog {
            id: "add_user_modal",
            class: "modal",
            class: if (props.is_user_modal_open)() { "modal-open" } else { "" },
            div { class: "modal-box",
                h3 { class: "text-lg font-bold", "Ajouter un utilisateur" }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Nom de l'utilisateur" }
                    input {
                        name: "user_name",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| user_name.set(event.value()),
                    }
                }
                form {
                    method: "dialog",
                    onclick: move |_| props.is_user_modal_open.set(false),
                    class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                    button { r#type: "button", "X" }
                }
                form { method: "dialog", class: "btn",
                    button {
                        r#type: "submit",
                        onclick: move |_| {
                            spawn(async move {
                                let creatable_user: CreatableUser = CreatableUser {
                                    name: user_name(),
                                    project_id: props.id,
                                };
                            });
                        },
                        "Enregistrer"
                    }
                }
            }
            form {
                method: "dialog",
                class: "modal-backdrop",
                onclick: move |_| props.is_user_modal_open.set(false),
                button { r#type: "button", "close" }
            }
        }
    }
}
