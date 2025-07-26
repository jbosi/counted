use dioxus::prelude::*;
use ui::{Expenses, Projects, Payments, Navbar};
use ui::route::Route;
use uuid::Uuid;
use shared::User;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        main {
            class: "bg-gray-50 min-h-screen flex flex-col items-center py-12 font-sans",
            link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" },
            Router::<Route> {}
        }
    }
}
