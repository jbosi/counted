use crate::modals::AddProjectModal;
use crate::projects::project::ProjectComponent;
use crate::utils::listen_to_sse_events;
use api::projects::projects_controller::get_projects;
use dioxus::prelude::*;
use shared::sse::EventSSE::{ProjectCreated, ProjectDeleted, ProjectModified};
use shared::ProjectDto;

#[component]
pub fn Projects() -> Element {
    let mut projects: Signal<Vec<ProjectDto>> = use_signal(|| vec![]);
    let mut modal_open = use_signal(|| false);
    let mut project_event_any = use_signal(|| String::new());

    let _ = use_resource(move || async move {
        // rerun the resource when event is fired
        let _ = project_event_any();

        match get_projects().await {
            Ok(items) => projects.set(items),
            Err(_) => (),
        }
    });

    // listen_to_sse_events(
    //     Vec::from([ProjectCreated, ProjectDeleted, ProjectModified]),
    //     project_event_any,payments_repository
    // );

    rsx! {
        div { class: "container p-4 max-w-md rounded-xl flex flex-col items-center",
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
                r#type: "button",
                class: "btn btn-circle btn-outline btn-lg",
                onclick: move |_| modal_open.set(true),
                "+"
            }
            AddProjectModal { modal_open }
        }
    }
}
