use api::account_projects::account_projects_controller::upsert_account_project;
use api::projects::projects_controller::add_project;
use api::users::users_controller::add_user;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    Account as AccountData, CreatableProject, CreatableUser, CreatableUserBatch,
    UpsertAccountProject,
};

use crate::common::{upsert_project, write_to_ls, LocalStorageState};
use crate::route::Route;

#[derive(Props, Clone, PartialEq)]
pub struct AddProjectModalProps {
    pub on_close: EventHandler<()>,
}

#[component]
pub fn AddProjectModal(props: AddProjectModalProps) -> Element {
    let mut project_name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut user_names: Signal<Vec<String>> =
        use_signal(|| vec![String::new(), String::new()]);
    let mut selected_idx: Signal<Option<usize>> = use_signal(|| None);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);

    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<AccountData>>>();
    let mut ls_ctx = use_context::<Signal<LocalStorageState>>();

    let on_submit = move |e: FormEvent| {
        e.prevent_default();

        // Client-side validation
        let name_val = project_name().trim().to_string();
        if name_val.is_empty() {
            error_msg.set(Some("Le nom du projet est requis.".into()));
            return;
        }
        let filled_names: Vec<String> = user_names()
            .into_iter()
            .filter(|n| !n.trim().is_empty())
            .map(|n| n.trim().to_string())
            .collect();
        if filled_names.is_empty() {
            error_msg.set(Some("Ajoutez au moins un participant.".into()));
            return;
        }
        if selected_idx().is_none() {
            error_msg.set(Some("Indiquez quel participant vous êtes.".into()));
            return;
        }

        loading.set(true);
        error_msg.set(None);

        let desc_val = description().trim().to_string();
        let is_auth = auth_ctx().is_some();
        let sel_idx = selected_idx();

        spawn(async move {
            // Step A — create project
            let project = match add_project(Json(CreatableProject {
                name: name_val,
                description: if desc_val.is_empty() { None } else { Some(desc_val) },
                currency: None,
            }))
            .await
            {
                Ok(p) => p,
                Err(e) => {
                    error_msg.set(Some(e.to_string()));
                    loading.set(false);
                    return;
                }
            };

            // Step B — create users
            let creatables: Vec<CreatableUser> = filled_names
                .iter()
                .map(|name| CreatableUser {
                    name: name.clone(),
                    project_id: project.id,
                    invited_email: if is_auth && name.contains('@') {
                        Some(name.clone())
                    } else {
                        None
                    },
                })
                .collect();

            let created_users = match add_user(Json(CreatableUserBatch::Multiple(creatables))).await {
                Ok(u) => u,
                Err(e) => {
                    error_msg.set(Some(e.to_string()));
                    loading.set(false);
                    return;
                }
            };

            // Step C — resolve which user the person selected
            let sel_name = sel_idx
                .and_then(|i| filled_names.get(i))
                .cloned()
                .unwrap_or_default();
            let user_id = created_users
                .iter()
                .find(|u| u.name.to_lowercase() == sel_name.to_lowercase())
                .map(|u| u.id);

            // Step D — persist to localStorage
            let mut state = ls_ctx();
            upsert_project(&mut state, project.id, user_id);
            write_to_ls(&state);
            ls_ctx.set(state);

            // Step E — if authenticated, also save to DB
            if is_auth {
                let _ = upsert_account_project(Json(UpsertAccountProject {
                    project_id: project.id,
                    user_id,
                }))
                .await;
            }

            // Step F — close modal and navigate
            props.on_close.call(());
            nav.push(Route::ProjectDetails { project_id: project.id });
        });
    };

    let on_close = props.on_close.clone();

    rsx! {
        div { class: "modal modal-open", role: "dialog",
            div { class: "modal-box max-w-sm relative",
                // Close button
                button {
                    r#type: "button",
                    class: "btn btn-ghost btn-sm btn-circle absolute right-2 top-2",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }

                h3 { class: "font-bold text-lg mb-4", "Nouveau projet" }

                if let Some(err) = error_msg() {
                    div { class: "alert alert-error text-sm mb-3", "{err}" }
                }

                form {
                    class: "flex flex-col gap-4",
                    onsubmit: on_submit,

                    // Project name
                    label { class: "form-control",
                        span { class: "label-text mb-1", "Nom du projet *" }
                        input {
                            class: "input input-bordered",
                            r#type: "text",
                            placeholder: "Mon voyage, Coloc 2024…",
                            value: "{project_name}",
                            oninput: move |e| project_name.set(e.value()),
                        }
                    }

                    // Description
                    label { class: "form-control",
                        span { class: "label-text mb-1", "Description" }
                        input {
                            class: "input input-bordered",
                            r#type: "text",
                            placeholder: "Optionnel",
                            value: "{description}",
                            oninput: move |e| description.set(e.value()),
                        }
                    }

                    // Participants
                    div { class: "flex flex-col gap-2",
                        span { class: "label-text font-medium", "Participants *" }

                        for i in 0..user_names().len() {
                            {
                                let name_val = user_names().get(i).cloned().unwrap_or_default();
                                let is_selected = selected_idx() == Some(i);
                                let can_remove = user_names().len() > 2;
                                rsx! {
                                    div { class: "flex gap-2 items-center",
                                        input {
                                            class: "input input-bordered flex-1 input-sm",
                                            r#type: "text",
                                            placeholder: "Nom ou email",
                                            value: "{name_val}",
                                            oninput: move |e| {
                                                let mut names = user_names.write();
                                                if let Some(slot) = names.get_mut(i) {
                                                    *slot = e.value();
                                                }
                                            },
                                        }
                                        button {
                                            r#type: "button",
                                            class: if is_selected { "btn btn-xs btn-primary shrink-0" } else { "btn btn-xs btn-outline shrink-0" },
                                            onclick: move |_| { selected_idx.set(Some(i)); },
                                            if is_selected { "✓ Moi" } else { "Moi ?" }
                                        }
                                        if can_remove {
                                            button {
                                                r#type: "button",
                                                class: "btn btn-xs btn-ghost btn-circle shrink-0",
                                                onclick: move |_| {
                                                    let mut names = user_names.write();
                                                    if i < names.len() {
                                                        names.remove(i);
                                                    }
                                                    // Adjust selected_idx if needed
                                                    if let Some(sel) = selected_idx() {
                                                        if sel == i {
                                                            selected_idx.set(None);
                                                        } else if sel > i {
                                                            selected_idx.set(Some(sel - 1));
                                                        }
                                                    }
                                                },
                                                "✕"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        button {
                            r#type: "button",
                            class: "btn btn-ghost btn-sm btn-outline self-start mt-1",
                            onclick: move |_| {
                                user_names.write().push(String::new());
                            },
                            "+ Ajouter un participant"
                        }
                    }

                    div { class: "modal-action mt-2",
                        button {
                            r#type: "button",
                            class: "btn",
                            onclick: move |_| props.on_close.call(()),
                            "Annuler"
                        }
                        button {
                            r#type: "submit",
                            class: "btn btn-primary",
                            disabled: loading(),
                            if loading() { "Création…" } else { "Créer" }
                        }
                    }
                }
            }

            // Backdrop — click to close
            div {
                class: "modal-backdrop",
                onclick: move |_| props.on_close.call(()),
            }
        }
    }
}
