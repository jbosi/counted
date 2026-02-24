use crate::modals::AddProjectModal;
use crate::projects::project::ProjectComponent;
use api::projects::projects_controller::get_projects;
use dioxus::prelude::*;
use shared::{Account, ProjectDto};

#[component]
pub fn Projects() -> Element {
    let mut projects: Signal<Vec<ProjectDto>> = use_signal(|| vec![]);
    let mut modal_open = use_signal(|| false);

    let auth_ctx = use_context::<Signal<Option<Account>>>();

    let greeting = match auth_ctx() {
        Some(ref account) => format!("Bonjour {}", account.display_name),
        None => "Bonjour".to_string(),
    };

    let _ = use_resource(move || async move {
        match get_projects().await {
            Ok(items) => projects.set(items),
            Err(_) => (),
        }
    });

    rsx! {
        div { class: "container p-4 max-w-md rounded-xl flex flex-col items-center",
            h1 { class: "text-4xl font-light mb-10", "{greeting}" }

            if auth_ctx().is_none() && projects().is_empty() {
                p { class: "text-base-content/60 mb-6",
                    "Sign in to create and manage your projects."
                }
                a { class: "btn btn-primary", href: "/login", "Sign in" }
            } else {
                div { class: "space-y-4 min-w-md",
                    for project in projects() {
                        ProjectComponent {
                            id: project.id,
                            title: project.name.to_string(),
                            current_reimbursements: 0,
                            total_reimbursements: 0,
                            description: project.description.clone().unwrap_or_else(|| "".to_string()),
                            currency: project.currency,
                            created_at: project.created_at,
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: "btn btn-circle btn-outline btn-lg",
                    onclick: move |_| modal_open.set(true),
                    "+"
                }
                AddProjectModal { modal_open }
            }
        }
    }
}
