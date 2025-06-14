#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use rust_decimal::Decimal;
use shared::{ExpenseSummary, FullGroupDetails, UserBalance};
use uuid::Uuid;

// --- POINT D'ENTRÉE ---
fn main() {
    // Lance l'application dans le navigateur
    dioxus::launch(App);
}

// --- ROUTAGE ---
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/group/:share_token")]
    GroupPage { share_token: Uuid },
}

// --- COMPOSANTS ---

fn App() -> Element {
    rsx! {
        // Le `Router` gère l'affichage des pages en fonction de l'URL
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "container mx-auto p-8",
            h1 { class: "text-3xl font-bold", "Bienvenue !" },
            p { "Pour commencer, rejoignez un groupe en utilisant une URL de partage." }
            // Vous pouvez ajouter ici un formulaire pour créer un groupe
        }
    }
}

#[component]
fn GroupPage(share_token: Uuid) -> Element {
    // `use_resource` pour charger les données de manière asynchrone depuis l'API
    // let details = use_resource(move || async move {
    //     fetch_group_details(share_token).await
    // });
    rsx! {
        p { "ok" }
    }
    // let details_read = details.read();

    // match &*details_read {
    //     Some(Ok(data)) => rsx! {
    //         div {
    //             class: "container mx-auto p-4 md:p-8",
    //             h1 { class: "text-4xl font-bold mb-4 text-gray-800", "{data.group.name}" },
    //             div {
    //                 class: "grid grid-cols-1 md:grid-cols-3 gap-8",
    //                 div {
    //                     class: "md:col-span-1 space-y-6",
    //                     BalancesCard { balances: data.balances.clone() },
    //                     MembersCard { users: data.users.clone() }
    //                 }
    //                 div {
    //                     class: "md:col-span-2",
    //                     ExpensesCard { expenses: data.expenses.clone() }
    //                 }
    //             }
    //         }
    //     },
    //     Some(Err(e)) => rsx! { div { class: "text-red-500 p-8", "Erreur de chargement: {e}" } },
    //     None => rsx! { div { class: "text-center p-8", "Chargement des données du groupe..." } },
    // }
}

#[component]
fn BalancesCard(balances: Vec<UserBalance>) -> Element {
    rsx! {
        div {
            class: "bg-white p-6 rounded-lg shadow-md",
            h2 { class: "text-2xl font-semibold mb-4 text-gray-700", "Équilibres" },
            ul {
                class: "space-y-2",
                for balance in balances {
                    {
                        let balance_val = balance.balance;
                        let balance_class = if balance_val.is_sign_positive() { "text-green-600" } else { "text-red-600" };
                        let sign = if balance_val.is_sign_positive() { "+" } else { "" };
    
                        rsx! {
                            li {
                                class: "flex justify-between items-center",
                                span { "{balance.user_name}" },
                                span { class: "font-mono font-bold {balance_class}", "{sign}{balance_val:.2} €" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MembersCard(users: Vec<shared::User>) -> Element {
    rsx! {
        div {
            class: "bg-white p-6 rounded-lg shadow-md",
            h2 { class: "text-2xl font-semibold mb-4 text-gray-700", "Membres" },
            ul {
                class: "space-y-2",
                for user in users {
                    li { "{user.name}" }
                }
            }
        }
    }
}

#[component]
fn ExpensesCard(expenses: Vec<ExpenseSummary>) -> Element {
    rsx! {
        div {
            class: "bg-white p-6 rounded-lg shadow-md",
            div {
                class: "flex justify-between items-center mb-4",
                h2 { class: "text-2xl font-semibold text-gray-700", "Dépenses" },
                button {
                    class: "bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-lg",
                    "Ajouter une dépense"
                }
            }
            div {
                class: "space-y-4",
                for expense in expenses {
                    div {
                        class: "p-4 border rounded-md flex justify-between",
                        div {
                           p { class: "font-bold", "{expense.description}" },
                           p { class: "text-sm text-gray-500", "Payé par {expense.paid_by_user_name}" }
                        }
                        div {
                            class: "text-right",
                            p { class: "font-mono font-bold text-lg", "{expense.amount:.2} €" },
                        }
                    }
                }
            }
        }
    }
}

// --- FONCTION D'APPEL API ---

async fn fetch_group_details(token: Uuid) -> Result<FullGroupDetails, reqwest::Error> {
    let url = format!("/api/group/{}", token);
    reqwest::get(&url).await?.json::<FullGroupDetails>().await
}
