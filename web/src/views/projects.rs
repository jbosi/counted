use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    let trips = vec![
        ("Voyage en Italie", 2, 6),
        ("Week-end à la mer", 4, 5),
        ("Randonnée en montagne", 1, 8),
        ("City-trip à Lisbonne", 5, 6),
    ];

    // let data = api::GetProjects().await.unwrap();

    rsx! {
            h1 {
                class: "text-4xl text-gray-700 font-light mb-10",
                "Bonjour Jonathan"
            }

            div {
                class: "space-y-4 min-w-md",
                
                {
                    trips.iter().map(|(title, current, total)| rsx!{
                        TripCard {
                            title: title.to_string(),
                            current_reimbursements: *current,
                            total_reimbursements: *total,
                            users: vec!["JB".to_string(), "AE".to_string(), "JC".to_string()],
                            more_users: 3
                        }
                    })
                }
            }
    }
}


#[derive(PartialEq, Props, Clone)]
struct TripCardProps {
    title: String,
    current_reimbursements: u32,
    total_reimbursements: u32,
    users: Vec<String>,
    more_users: u32,
}

fn TripCard(props: TripCardProps) -> Element {
    // Calcul du pourcentage pour la barre de progression
    let progress_percentage = (props.current_reimbursements as f32 / props.total_reimbursements as f32) * 100.0;

    rsx! {
        div {
            class: "bg-white p-6 rounded-2xl shadow-sm w-full max-w-md",

            // Entête de la carte (Titre et menu ...)
            div {
                class: "flex justify-between items-start mb-2",
                div {
                    h2 { class: "font-bold text-lg text-gray-800", "{props.title}" }
                    p { class: "text-gray-400 text-sm", "Description" }
                }
                button { class: "text-gray-400 font-bold text-xl", "..." }
            }

            // Section de la barre de progression
            div {
                class: "my-4",
                div {
                    class: "flex justify-between text-sm text-gray-600 mb-1",
                    span { "Remboursements" }
                    span { class: "font-semibold",
                        "{props.current_reimbursements}/{props.total_reimbursements}"
                    }
                }
                // La barre de progression
                div {
                    class: "bg-gray-200 rounded-full h-1.5",
                    div {
                        class: "bg-gray-800 h-1.5 rounded-full",
                        style: "width: {progress_percentage}%",
                    }
                }
            }

            // Pied de la carte (Bouton et avatars)
            div {
                class: "flex justify-between items-center mt-6",
                button {
                    class: "bg-gray-100 text-gray-500 font-semibold py-2 px-6 rounded-full text-sm",
                    "Clôturé"
                }
                div {
                    class: "flex items-center",
                    // On affiche les avatars des users
                    {props.users.iter().map(|initials| rsx!{
                        Avatar { text: initials.clone() }
                    })}
                    // On affiche le nombre de users supplémentaires
                    {if props.more_users > 0 {
                        rsx! { Avatar { text: format!("+{}", props.more_users) } }
                    } else {
                        rsx! { "" }
                    }}
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct AvatarProps {
    text: String,
}

fn Avatar(props: AvatarProps) -> Element {
    rsx! {
        div {
            // Le "-ml-3" permet de superposer les avatars
            class: "flex items-center justify-center w-9 h-9 bg-gray-300 rounded-full border-2 border-white text-white font-bold text-xs -ml-3",
            "{props.text}"
        }
    }
}