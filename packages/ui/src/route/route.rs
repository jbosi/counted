use crate::auth::login::Login;
use crate::auth::register::Register;
use crate::common::Navbar;
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    #[route("/projects/:project_id")]
    #[route("/projects/:project_id/expenses/:expense_id")]
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
pub fn WebNavbar() -> Element {
    rsx! {
        Navbar {
        }

        Outlet::<Route> {}
    }
}
