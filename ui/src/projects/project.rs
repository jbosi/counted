use crate::common::{Avatar, Toast};
use crate::modals::{AddProjectModal, UpdateProjectModal};
use crate::route::Route;
use api::projects::delete_project_by_id;
use api::users::get_users_by_project_id;
use dioxus::document::{document, Eval};
use dioxus::hooks::{use_resource, use_signal};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use shared::api::{ApiError, ApiState};
use shared::{Project, UpdatableProject, User};
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct ProjectProps {
    id: Uuid,
    title: String,
    current_reimbursements: u32,
    total_reimbursements: u32,
    // users: Vec<String>,
    // more_users: u32,
    description: Option<String>,
}

#[component]
pub fn Project(props: ProjectProps) -> Element {
    let mut users: Signal<Vec<User>> = use_signal(|| vec![]);
    let mut more_users: Signal<i32> = use_signal(|| 0);
    let mut update_project_modal_open: Signal<bool> = use_signal(|| false);
    let mut api_project_delete_state = use_signal(|| ApiState::<()>::Loading);

    let _ = use_resource(move || async move {
        match get_users_by_project_id(props.id).await {
            Ok(u) => {
                if u.len() > 3 {
                    more_users.set((u.len() - 3).try_into().unwrap());
                }
                return users.set(u);
            }
            Err(_) => (),
        }
    });

    let progress_percentage: u32 =
        ((props.current_reimbursements as f32 / props.total_reimbursements as f32) * 100.0).round()
            as u32;

    let description = match &props.description {
        Some(desc) => desc.clone(),
        None => "".to_string(),
    };

    rsx! {
        section {
            class: "card bg-base-100 w-96 shadow-sm",
            onclick: move |_| {
                navigator()
                    .push(Route::Expenses {
                        project_id: props.id,
                    });
            },
            div { class: "card-body",
                div {
                    div {
                        class: "flex flex-row justify-between",
                        h2 { class: "card-title", "{props.title}" }
                        details {
                            class: "dropdown",
                            onclick: move |event| {
                                event.stop_propagation();
                            },
                            summary {
                                class: "btn btn-ghost btn-circle",
                                "..."
                            },
                            ul {
                                class: "menu dropdown-content bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm",
                                popover: "",
                                id: "popover-project-dot",
                                style: "position-anchor:--anchor-project-dot",
                                li {
                                    button {
                                        class: "btn btn-ghost",
                                        onclick: move |event| async move {
                                            close_dropdown().await.unwrap_or("".into());

                                            update_project_modal_open.set(true);
                                        },
                                        "Editer"
                                    }
                                }
                                li {
                                    button {
                                        class: "btn btn-ghost",
                                        onclick: move |_| {
                                            spawn(async move {
                                                close_dropdown().await.unwrap_or("".into());

                                                match delete_project_by_id(props.id).await {
                                                    Ok(()) => api_project_delete_state.set(ApiState::Success(())),
                                                    Err(error) => api_project_delete_state.set(ApiState::Error(ApiError(error.to_string())))
                                                };
                                            });
                                        },
                                        "Supprimer"
                                    }
                                }
                            }
                        }
                    }
                    p { "{description}" }
                }
                div { class: "flex justify-between",
                    span { "Remboursements" }
                    span { "{props.current_reimbursements}/{props.total_reimbursements}" }
                }
                progress {
                    class: "progress",
                    value: "{progress_percentage}",
                    max: 100,
                }
                div { class: "card-actions justify-between",
                    div { class: "flex gap-2 items-center",
                        div { class: "status status-success" }
                        span { "En cours" }
                    }
                    div { class: "",
                        for user in users() {
                            Avatar { initials: user.name.get(0..2).unwrap_or("") }
                        }
                        if more_users() > 0 {
                            Avatar { initials: format!("+{}", more_users) }
                        } else {
                            ""
                        }
                    }
                }
                if let ApiState::Error(error) = api_project_delete_state() {
                    Toast {
                        error: error,
                        onclick: move |event| {
                            api_project_delete_state.set(ApiState::None)
                        }
                    }
                }
            }
        }
        UpdateProjectModal {
            modal_open: update_project_modal_open,
            current_project: UpdatableProject { id: props.id, currency: Some("EUR".to_string()), description: props.description, name: Some(props.title) }
        }
    }
}

fn close_dropdown() -> Eval {
    document::eval(
        "document.activeElement.closest('.dropdown').removeAttribute('open'); return null",
    )
}
