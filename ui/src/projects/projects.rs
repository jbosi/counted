use crate::modals::AddProjectModal;
use crate::projects::project::ProjectComponent;
use api::projects::get_projects;
use dioxus::prelude::*;
use shared::ProjectDto;

#[component]
pub fn Projects() -> Element {
    let mut projects: Signal<Vec<ProjectDto>> = use_signal(|| vec![]);
    let mut modal_open = use_signal(|| false);

    let _ = use_resource(move || async move {
        match get_projects().await {
            Ok(items) => projects.set(items),
            Err(_) => (),
        }
    });

    rsx! {
        div {
            class: "container p-4 max-w-md rounded-xl flex flex-col items-center",
            h1 { class: "text-4xl font-light mb-10", "Bonjour Jonathan" }

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
                type: "button",
                class: "btn btn-circle btn-outline btn-lg",
                onclick: move |_| modal_open.set(true),
                "+"
            }
            AddProjectModal { modal_open }
        }
    }
}
