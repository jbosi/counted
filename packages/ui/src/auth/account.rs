use api::auth::auth_controller::logout;
use dioxus::prelude::*;
use shared::Account as AccountData;

use crate::route::Route;

#[component]
pub fn Account() -> Element {
    let nav = use_navigator();
    let mut auth_ctx = use_context::<Signal<Option<AccountData>>>();
    let mut loading = use_signal(|| false);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);

    let account = auth_ctx();

    // Redirect unauthenticated users
    if account.is_none() {
        nav.push(Route::Login {});
        return rsx! {};
    }

    let account: AccountData = account.unwrap();

    let on_logout = move |_| {
        async move {
            loading.set(true);
            match logout().await {
                Ok(_) => {
                    auth_ctx.set(None);
                    nav.push(Route::ProjectsList {});
                }
                Err(e) => {
                    error_msg.set(Some(e.to_string()));
                    loading.set(false);
                }
            }
        }
    };

    rsx! {
        div { class: "container p-4 max-w-md mx-auto flex flex-col gap-4",
            // Header
            div { class: "navbar px-0",
                div { class: "navbar-start",
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle",
                        onclick: move |_| { nav.push(Route::ProjectsList {}); },
                        svg {
                            class: "w-5 h-5",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            view_box: "0 0 24 24",
                            path { d: "M15 18l-6-6 6-6" }
                        }
                    }
                }
                div { class: "navbar-center",
                    h1 { class: "text-xl font-bold", "Mon compte" }
                }
                div { class: "navbar-end" }
            }

            if let Some(err) = error_msg() {
                div { class: "alert alert-error text-sm", "{err}" }
            }

            div { class: "card bg-base-100 shadow",
                div { class: "card-body gap-4",
                    div { class: "flex flex-col gap-1",
                        span { class: "text-xs text-base-content/60 uppercase font-semibold", "Nom" }
                        span { class: "font-medium", "{account.display_name}" }
                    }
                    div { class: "divider my-0" }
                    div { class: "flex flex-col gap-1",
                        span { class: "text-xs text-base-content/60 uppercase font-semibold", "Email" }
                        span { class: "font-medium", "{account.email}" }
                    }
                    div { class: "divider my-0" }
                    div { class: "flex flex-col gap-1",
                        span { class: "text-xs text-base-content/60 uppercase font-semibold", "Membre depuis" }
                        span { class: "font-medium", "{account.created_at.format(\"%d/%m/%Y\")}" }
                    }
                    div { class: "card-actions justify-end mt-2",
                        button {
                            r#type: "button",
                            class: "btn btn-error btn-outline",
                            disabled: loading(),
                            onclick: on_logout,
                            if loading() { "Déconnexion…" } else { "Se déconnecter" }
                        }
                    }
                }
            }
        }
    }
}
