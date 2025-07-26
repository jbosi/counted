use api::projects::add_project;
use dioxus::prelude::*;
use shared::CreatableProject;
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct AddProjectModalProps {
    modal_open: Signal<bool>,
}
#[component]
pub fn AddProjectModal(mut props: AddProjectModalProps) -> Element {
    let mut project_name: Signal<String> = use_signal(|| "".to_string());
    let mut project_description: Signal<Option<String>> = use_signal(|| None);

    rsx! {
        dialog {
            id: "add_project_modal",
            class: "modal",
            class: if (props.modal_open)() { "modal-open" } else { "" },
            div { class: "modal-box",
                h3 { class: "text-lg font-bold", "Ajouter un projet" }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Nom du projet" }
                    input {
                        name: "project_name",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| project_name.set(event.value()),
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Description du projet" }
                    input {
                        name: "project_description",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| project_description.set(Some(event.value())),
                    }
                }
                form {
                    method: "dialog",
                    onclick: move |_| props.modal_open.set(false),
                    class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                    button { type: "button", "X" }
                }
                form { method: "dialog", class: "btn",
                    button {
                        r#type: "submit",
                        onclick: move |_| {
                            spawn(async move {
                                let creatable_project: CreatableProject = CreatableProject {
                                    name: project_name(),
                                    description: project_description(),
                                };
                                add_project(creatable_project).await.expect("Failed to add new project");
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
                button { type: "button", "close" }
            }
        }
    }
}
