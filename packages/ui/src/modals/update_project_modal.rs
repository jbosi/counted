use api::projects::projects_controller::{add_project, update_project_by_id};
use dioxus::prelude::*;
use shared::{CreatableProject, EditableProject};
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct UpdateProjectModalProps {
    modal_open: Signal<bool>,
    current_project: EditableProject,
}
#[component]
pub fn UpdateProjectModal(mut props: UpdateProjectModalProps) -> Element {
    let mut project_name: Signal<Option<String>> =
        use_signal(|| props.current_project.name.clone());
    let mut project_description: Signal<Option<String>> =
        use_signal(|| props.current_project.description.clone());

    rsx! {
        dialog {
            id: "add_project_modal",
            class: "modal",
            class: if (props.modal_open)() { "modal-open" } else { "" },
            div { class: "modal-box",
                h3 { class: "text-lg font-bold", "Modifier un projet" }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Nom du projet" }
                    input {
                        name: "project_name",
                        r#type: "text",
                        class: "input",
                        initial_value: project_name(),
                        oninput: move |event| project_name.set(Some(event.value())),
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Description du projet" }
                    input {
                        name: "project_description",
                        r#type: "text",
                        class: "input",
                        initial_value: project_description(),
                        oninput: move |event| project_description.set(Some(event.value())),
                    }
                }
                form {
                    method: "dialog",
                    onclick: move |_| props.modal_open.set(false),
                    class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                    button { r#type: "button", "X" }
                }
                form { method: "dialog", class: "btn",
                    button {
                        r#type: "submit",
                        onclick: move |_| {
                            spawn(async move {
                                let updatable_project: EditableProject = EditableProject {
                                    id: props.current_project.id,
                                    name: project_name(),
                                    description: project_description(),
                                    currency: None,
                                };
                                // update_project_by_id(updatable_project)
                                //     .await
                                //     .expect("Failed to update project");
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
                button { r#type: "button", "close" }
            }
        }
    }
}
