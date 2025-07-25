use dioxus::prelude::*;
use ui::Navbar;
use uuid::Uuid;
use shared::User;
use views::{Expenses, Projects, Payments};

mod views;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Projects {},
    #[route("/projects/:project_id")]
    Expenses { project_id: Uuid },
    #[route("/projects/:project_id/expenses/:expense_id")]
    Payments { project_id: Uuid, expense_id: i32 },
}

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


/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Projects {},
                "Projects"
            }
        }

        Outlet::<Route> {}
    }
}
