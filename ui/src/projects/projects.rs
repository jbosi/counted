use crate::common::Avatar;
use crate::modals::AddProjectModal;
use crate::projects::Project;
use api::projects::get_projects;
use api::users::get_users_by_project_id;
use dioxus::prelude::*;
use shared::{Project, User};
use uuid::Uuid;

#[component]
pub fn Projects() -> Element {
    let mut projects: Signal<Vec<Project>> = use_signal(|| vec![]);
    let mut modal_open = use_signal(|| false);

    use_resource(move || async move {
        match get_projects().await {
            Ok(items) => projects.set(items),
            Err(_) => (),
        }
    });

    rsx! {
        h1 { class: "text-4xl text-gray-700 font-light mb-10", "Bonjour Jonathan" }

        div { class: "space-y-4 min-w-md",
            for project in projects() {
                Project {
                    id: project.id,
                    title: project.name.to_string(),
                    current_reimbursements: 0,
                    total_reimbursements: 0,
                    description: project.description.clone().unwrap_or_else(|| "".to_string()),
                }
            }
        }
        button {
            type: "button",
            class: "btn btn-circle btn-outline btn-lg bg-base-100",
            onclick: move |_| modal_open.set(true),
            "+"
        }
        AddProjectModal { modal_open }
    }
}
