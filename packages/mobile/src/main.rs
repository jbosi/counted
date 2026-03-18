use api::auth::auth_controller::me;
use dioxus::prelude::*;
use shared::Account;
use ui::common::{read_from_ls, LocalStorageState};
use ui::route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::logger::initialize_default();
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    let auth: Signal<Option<Account>> = use_context_provider(|| Signal::new(None));
    let _ls: Signal<LocalStorageState> = use_context_provider(|| Signal::new(read_from_ls()));

    use_effect(move || {
        let mut auth = auth;
        spawn(async move {
            if let Ok(account) = me().await {
                auth.set(account);
            }
        });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        main {
            "data-theme": "cupcake",
            class: "min-h-screen flex flex-col items-center",
            Router::<Route> {}
        }
    }
}
