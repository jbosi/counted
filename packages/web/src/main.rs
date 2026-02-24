use api::auth::auth_controller::me;
use dioxus::prelude::*;
use shared::Account;
use ui::route::Route;

use shared::sse::EventSSE;

#[cfg(feature = "server")]
use api::sse::sse_handler;
#[cfg(feature = "server")]
use api::sse::{AppState, Broadcaster};
#[cfg(feature = "server")]
use axum::routing::get;
#[cfg(feature = "server")]
use axum::{Router, ServiceExt};
use web_sys::wasm_bindgen::prelude::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::EventSource;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::initialize_default();
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    // Global auth state — populated on mount by calling /api/v1/auth/me
    let auth: Signal<Option<Account>> = use_context_provider(|| Signal::new(None));

    use_effect(move || {
        let mut auth = auth;
        spawn(async move {
            if let Ok(account) = me().await {
                auth.set(account);
            }
        });
    });

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        main {
            "data-theme": "cupcake",
            class: "min-h-screen flex flex-col items-center",
            script { src: "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" }
            link {
                rel: "stylesheet",
                href: "https://cdn.jsdelivr.net/npm/daisyui@5",
            }
            link {
                rel: "stylesheet",
                href: "https://cdn.jsdelivr.net/npm/daisyui@5/themes.css",
            }
            Router::<Route> {}
        }
    }
}
