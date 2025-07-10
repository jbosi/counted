use dioxus::prelude::*;
use uuid::Uuid;
use api::add_user;
use shared::{CreatableUser};

#[derive(PartialEq, Props, Clone)]
pub struct AddUserModalProps {
    id: Uuid,
    modal_open: Signal<bool>
}
#[component]
pub fn AddUserModal(mut props: AddUserModalProps) -> Element {
    let mut user_name: Signal<String> = use_signal(|| "".to_string());

    rsx! {
        dialog {
            id: "add_user_modal",
            class: "modal",
            class: if (props.modal_open)() { "modal-open" } else { "" },
            div {
                class: "modal-box",
                h3 {
                    class: "text-lg font-bold",
                    "Ajouter un utilisateur"
                }
                fieldset {
                    class:"fieldset",
                    legend {
                        class: "fieldset-legend",
                        "Nom de l'utilisateur"
                    }
                    input {
                        name: "user_name",
                        type: "text",
                        class: "input",
                        oninput: move |event| user_name.set(event.value())
                    },
                }
                form {
                    method: "dialog",
                     onclick: move |_| props.modal_open.set(false),
                    class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                    button {
                        "X"
                    }
                }
                form {
                    method: "dialog",
                    class: "btn",
                    button {
                        r#type: "submit",
                        onclick: move |_| {
                        spawn(async move {
                            let creatable_user: CreatableUser = CreatableUser {
                                name: user_name(),
                                project_id: props.id,
                            };

                            add_user(creatable_user).await.expect("Failed to add new user to this project");
                            props.modal_open.set(false)
                        });
                    },
                        "Enregistrer"
                    }
                }
            }
            form {
                method: "dialog",
                class: "modal-backdrop",
                onclick: move |_| props.modal_open.set(false),
                button {
                    "close"
                }
            }
        }
    }
}