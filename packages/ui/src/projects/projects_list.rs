use api::account_projects::account_projects_controller::get_account_projects;
use api::projects::projects_controller::get_projects_by_ids;
use api::users::users_controller::get_users_by_project_id;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, BatchProject, ProjectDto, ProjectStatus, User};
use uuid::Uuid;

use crate::common::{
    initials, upsert_project, user_color_class, write_to_ls, Avatar, EmptyMagnifyingGlassIllustration, LocalStorageState,
};
use crate::icons::{SettingsIcon, UserIcon};
use crate::projects::AddProjectModal;
use crate::route::Route;

#[component]
pub fn ProjectsList() -> Element {
    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut show_archived = use_signal(|| false);
    let mut show_add_project = use_signal(|| false);

    // On mount: if authenticated, sync server projects into localStorage
    use_effect(move || {
        if auth_ctx().is_some() {
            let mut ls_ctx = ls_ctx;
            spawn(async move {
                if let Ok(server_projects) = get_account_projects().await {
                    let mut state = ls_ctx();
                    for sp in server_projects {
                        upsert_project(&mut state, sp.project_id, sp.user_id);
                    }
                    write_to_ls(&state);
                    ls_ctx.set(state);
                }
            });
        }
    });

    let projects = use_resource(move || async move {
        let ids: Vec<Uuid> = ls_ctx().projects.iter().map(|p| p.project_id).collect();
        if ids.is_empty() {
            return Ok(vec![]);
        }
        get_projects_by_ids(Json(BatchProject { ids })).await
    });

    let project_count = move || match &*projects.read() {
        Some(Ok(list)) => list.len(),
        _ => 0,
    };

    rsx! {
        div { class: "container overflow-auto p-4 max-w-md w-full mx-auto flex flex-col gap-4 pb-24",
            // Header
            div { class: "navbar px-0",
                div { class: "navbar-start",
                    h1 { class: "text-xl font-bold", "Counted" }
                }
                div { class: "navbar-end flex gap-1",
                    // User icon
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle",
                        onclick: move |_| {
                            if auth_ctx().is_some() {
                                nav.push(Route::Account {});
                            } else {
                                nav.push(Route::Login {});
                            }
                        },
                        UserIcon {}
                    }

                    // Settings dropdown
                    details { class: "dropdown dropdown-end",
                        summary { class: "btn btn-ghost btn-circle", SettingsIcon {} }
                        ul { class: "menu dropdown-content bg-base-100 rounded-box w-56 shadow z-10 p-2",
                            li {
                                label { class: "flex items-center gap-3 cursor-pointer",
                                    span { class: "flex-1", "Afficher les archivés" }
                                    input {
                                        r#type: "checkbox",
                                        class: "toggle toggle-sm",
                                        checked: show_archived(),
                                        oninput: move |e| show_archived.set(e.checked()),
                                    }
                                }
                            }
                            li {
                                button { class: "btn btn-ghost btn-sm justify-start btn-disabled",
                                    "Importer depuis Tricount"
                                }
                            }
                        }
                    }
                }
            }

            // Stats
            div { class: "stats shadow w-full",
                div { class: "stat",
                    div { class: "stat-title", "Projets" }
                    div { class: "stat-value", "{project_count()}" }
                }
            }

            // Project list
            match &*projects.read() {
                None => rsx! {
                div { class: "flex justify-center py-8",
                    span { class: "loading loading-spinner loading-md" }
                }
            },
                Some(Err(e)) => rsx! {
                div { class: "alert alert-error", "{e}" }
            },
                Some(Ok(list)) => {
                    let filtered: Vec<ProjectDto> = list
                        .iter()
                        .filter(|p| show_archived() || p.status != ProjectStatus::Archived)
                        .cloned()
                        .collect();
                    if filtered.is_empty() {
                        rsx! {
                div { class: "flex flex-col items-center gap-2 py-12 text-base-content/60",
                    EmptyMagnifyingGlassIllustration {}
                    span { class: "font-semibold", "Aucun projet" }
                    span { class: "text-sm text-center", "Créez un projet en cliquant sur le bouton ci-dessous" }
                }
            }
                    } else {
                        rsx! {
                div { class: "flex flex-col gap-3",
                    for project in filtered {
                        ProjectCard { project: project.clone() }
                    }
                }
            }
                    }
                }
            }

            // FAB
            div { class: "fixed bottom-6 right-6",
                button {
                    r#type: "button",
                    class: "btn btn-circle btn-lg btn-primary shadow-lg",
                    "aria-label": "Ajouter un projet",
                    onclick: move |_| {
                        show_add_project.set(true);
                    },
                    "+"
                }
            }

            if show_add_project() {
                AddProjectModal { on_close: move |_| show_add_project.set(false) }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ProjectCard
// ---------------------------------------------------------------------------

#[derive(PartialEq, Props, Clone)]
struct ProjectCardProps {
    project: ProjectDto,
}

#[component]
fn ProjectCard(props: ProjectCardProps) -> Element {
    let nav = use_navigator();
    let project = props.project.clone();
    let project_id = project.id;

    let users = use_resource(move || async move { get_users_by_project_id(project_id).await });

    rsx! {
        div {
            class: "card bg-base-100 shadow-sm cursor-pointer hover:shadow-md transition-shadow",
            onclick: move |_| {
                nav.push(Route::ProjectDetails {
                    project_id,
                });
            },
            div { class: "card-body p-4 gap-2",
                div { class: "flex items-start justify-between gap-2",
                    div { class: "flex flex-col gap-1 min-w-0",
                        h2 { class: "card-title text-base truncate", "{project.name}" }
                        if let Some(desc) = &project.description {
                            if !desc.is_empty() {
                                p { class: "text-xs text-base-content/60 truncate",
                                    "{desc}"
                                }
                            }
                        }
                    }
                    span { class: "text-xs font-mono text-base-content/50 shrink-0",
                        "{project.currency}"
                    }
                }
                div { class: "flex items-center justify-between",
                    StatusBadge { status: project.status.clone() }
                    // Avatar group
                    match &*users.read() {
                        Some(Ok(user_list)) if !user_list.is_empty() => {
                            rsx! {
                        AvatarGroup { users: user_list.clone() }
                    }
                        }
                        _ => rsx! {},
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// StatusBadge
// ---------------------------------------------------------------------------

#[derive(PartialEq, Props, Clone)]
struct StatusBadgeProps {
    status: ProjectStatus,
}

#[component]
fn StatusBadge(props: StatusBadgeProps) -> Element {
    let (dot_class, label) = match props.status {
        ProjectStatus::Ongoing => ("status-success", "En cours"),
        ProjectStatus::Closed => ("status-warning", "Clôturé"),
        ProjectStatus::Archived => ("status-neutral", "Archivé"),
    };
    rsx! {
        div { class: "flex gap-1 items-center text-xs",
            div { class: "status {dot_class}" }
            span { "{label}" }
        }
    }
}

// ---------------------------------------------------------------------------
// AvatarGroup
// ---------------------------------------------------------------------------

#[derive(PartialEq, Props, Clone)]
struct AvatarGroupProps {
    users: Vec<User>,
}

#[component]
fn AvatarGroup(props: AvatarGroupProps) -> Element {
    const MAX: usize = 4;
    let shown: Vec<&User> = props.users.iter().take(MAX).collect();
    let overflow = props.users.len().saturating_sub(MAX);

    rsx! {
        div { class: "avatar-group -space-x-3",
            for user in shown {
                Avatar {
                    initials: initials(&user.name),
                    size: 8,
                    color_class: user_color_class(user.id).to_string(),
                }
            }
            if overflow > 0 {
                div { class: "avatar avatar-placeholder",
                    div { class: "bg-neutral-focus text-neutral-content w-8 rounded-full",
                        span { class: "text-xs", "+{overflow}" }
                    }
                }
            }
        }
    }
}
