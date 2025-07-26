use crate::common::Avatar;
use crate::route::Route;
use api::get_users_by_project_id;
use dioxus::hooks::{use_resource, use_signal};
use dioxus::prelude::*;
use shared::User;
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

pub fn Project(props: ProjectProps) -> Element {
    let mut users: Signal<Vec<User>> = use_signal(|| vec![]);
    let mut more_users: Signal<i32> = use_signal(|| 0);

    use_resource(move || async move {
        match get_users_by_project_id(props.id).await {
            Ok(u) => {
                if u.len() > 3 {
                    more_users.set((u.len() - 3).try_into().unwrap());
                }
                return users.set(u);
            },
            Err(_) => ()
        }
    });

    let progress_percentage: u32 = ((props.current_reimbursements as f32 / props.total_reimbursements as f32) * 100.0).round() as u32;

    let description = match &props.description {
        Some(desc) => desc.clone(),
        None => "".to_string(),
    };

    rsx! {
        div {
            class: "card bg-base-100 w-96 shadow-sm",
            onclick: move |_| {
                navigator()
                    .push(Route::Expenses {
                        project_id: props.id,
                    });
            },
            div { class: "card-body",
                div {
                    h2 { class: "card-title", "{props.title}" }
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
            }
        }
    }
}