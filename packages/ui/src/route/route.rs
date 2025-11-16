use crate::common::Navbar;
use crate::expenses::Expenses;
use crate::payments::Payments;
use crate::projects::Projects;
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Projects {},
    #[route("/projects/:project_id")]
    Expenses { project_id: Uuid },
    #[route("/projects/:project_id/expenses/:expense_id")]
    Payments { project_id: Uuid, expense_id: i32 },
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
pub fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            Link { to: Route::Projects {}, "Projects" }
        }

        Outlet::<Route> {}
    }
}
