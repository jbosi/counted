use dioxus::prelude::*;
use shared::{ReimbursementSuggestion, User};

use crate::{
    common::{initials, user_color_class, Avatar, CheckMarkIllustration},
    icons::{DollarIcon, RightArrowIcon},
};

#[derive(PartialEq, Props, Clone)]
pub struct ReimbursementsTabProps {
    pub suggestions: Vec<ReimbursementSuggestion>,
    pub users: Vec<User>,
    pub currency: String,
    pub on_reimburse: EventHandler<ReimbursementSuggestion>,
}

#[component]
pub fn ReimbursementsTab(props: ReimbursementsTabProps) -> Element {
    let currency = props.currency.clone();

    if props.suggestions.is_empty() {
        return rsx! {
            div { class: "flex flex-col items-center gap-2 py-12 text-base-content/60",
                CheckMarkIllustration {}
                span { class: "font-bold text-base-content", "Les comptes sont bons !" }
                span { class: "text-sm text-center",
                    "Des suggestions de remboursement seront proposées ici si les comptes ne sont pas équilibrés"
                }
            }
        };
    }

    rsx! {
        ul { class: "flex flex-col gap-3",
            for suggestion in props.suggestions.iter() {
                {
                    let debtor = props.users.iter().find(|u| u.id == suggestion.user_id_debtor);
                    let payer = props.users.iter().find(|u| u.id == suggestion.user_id_payer);
                    let debtor_name = debtor.map(|u| u.name.as_str()).unwrap_or("?");
                    let payer_name = payer.map(|u| u.name.as_str()).unwrap_or("?");
                    let debtor_initials = initials(debtor_name);
                    let payer_initials = initials(payer_name);
                    let debtor_color = debtor
                        .map(|u| user_color_class(u.id))
                        .unwrap_or("bg-neutral");
                    let payer_color = payer.map(|u| user_color_class(u.id)).unwrap_or("bg-neutral");
                    let amount = suggestion.amount; // Avatars with arrow
                    let curr = currency.clone();
                    let suggestion_clone = suggestion.clone();
                    rsx! { // Info
                    li { class: "bg-base-100 rounded-lg shadow-sm p-4 flex items-center gap-3", // Avatars with arrow
                        // Avatars with arrow
                        div { class: "flex items-center gap-1 shrink-0",
                            Avatar {
                                initials: debtor_initials,
                                size: 8,
                                color_class: debtor_color.to_string(),
                            }
                            RightArrowIcon {} // Info
                            Avatar {
                                initials: payer_initials,
                                size: 8,
                                color_class: payer_color.to_string(),
                            }
                        }
                        // Info
                        div { class: "flex-1 min-w-0",
                            p { class: "text-sm font-medium truncate", "{debtor_name} → {payer_name}" }
                            p { class: "text-xs font-semibold text-base-content/60 uppercase", "{amount:.2} {curr}" }
                        }
                        // Record button
                        button {
                            r#type: "button",
                            class: "btn btn-circle btn-sm btn-ghost",
                            title: "Ajouter un remboursement",
                            onclick: move |_| props.on_reimburse.call(suggestion_clone.clone()),
                            DollarIcon {}
                        }
                    }
                }
                }
            }
        }
    }
}
