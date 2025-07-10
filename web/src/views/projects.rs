use crate::Route;
use api::{get_projects, get_users_by_project_id};
use dioxus::prelude::*;
use shared::{Project, User};
use ui::Avatar;
use uuid::Uuid;

#[component]
pub fn Projects() -> Element {
    let mut trips: Signal<Vec<Project>> = use_signal(|| vec![]);

    let _ = use_resource(move || async move {
        match get_projects().await {
            Ok(items) => trips.set(items),
            Err(_) => ()
        }
    });

    rsx! {
            h1 {
                class: "text-4xl text-gray-700 font-light mb-10",
                "Bonjour Jonathan"
            }

            div {
                class: "space-y-4 min-w-md",

                {
                    trips.iter().map(|trip| rsx!{
                        Project {
                            id: trip.id,
                            title: trip.name.to_string(),
                            current_reimbursements: 0,
                            total_reimbursements: 0,
                            // users: vec!["JB".to_string(), "AE".to_string(), "JC".to_string()],
                            // more_users: 3,
                            description: "A card component has a figure, a body part, and inside body there are title and actions parts",
                        }
                    })
                }
            }
    }
}


#[derive(PartialEq, Props, Clone)]
struct ProjectProps {
    id: Uuid,
    title: String,
    current_reimbursements: u32,
    total_reimbursements: u32,
    // users: Vec<String>,
    // more_users: u32,
    description: Option<String>,
}

fn Project(props: ProjectProps) -> Element {
    let mut users: Signal<Vec<User>> = use_signal(|| vec![]);
    let mut more_users: Signal<i32> = use_signal(|| 0);

    let _ = use_resource(move || async move {
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

    // Calcul du pourcentage pour la barre de progression
    let progress_percentage: u32 = ((props.current_reimbursements as f32 / props.total_reimbursements as f32) * 100.0).round() as u32;
    
    let description = match &props.description {
        Some(desc) => desc.clone(),
        None => "".to_string(),
    };
    
    rsx! {
        div {
            class: "card bg-base-100 w-96 shadow-sm",
            onclick: move |_| {
                navigator().push(Route::Expenses { id: props.id });
            },
            div {
                class: "card-body",
                div {
                    h2 {
                        class: "card-title",
                        "{props.title}"
                    }
                    p { "{description}" }
                }
                div {
                    class: "flex justify-between",
                    span { "Remboursements" }
                    span { "{props.current_reimbursements}/{props.total_reimbursements}" }
                }
                progress {
                    class: "progress",
                    value: "{progress_percentage}",
                    max: 100
                }
                div {
                    class: "card-actions justify-between",
                    div {
                        class: "flex gap-2 items-center",
                        div {
                            class: "status status-success",
                        } 
                        span { "En cours" },
                    }
                    div {
                        class: "",
                        // On affiche les avatars des users
                        for user in users() {
                            Avatar { initials: user.name.get(0..2).unwrap_or("") }
                        }
                        // On affiche le nombre de users supplémentaires
                        {
                            if more_users() > 0 {
                                rsx! { Avatar { initials: format!("+{}", more_users) } }
                            } else {
                                rsx! { "" }
                            }
                        }
                    }
                }
            }
        }
    }
}

