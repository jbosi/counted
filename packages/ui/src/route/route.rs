use crate::auth::{account::Account, login::Login, register::Register};
use crate::common::Navbar;
use crate::payments::PaymentPage;
use crate::project_details::ProjectDetails;
use crate::projects::ProjectsList;
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    ProjectsList {},
    #[route("/projects/:project_id")]
    ProjectDetails { project_id: Uuid },
    #[route("/projects/:project_id/expenses/:expense_id")]
    PaymentPage { project_id: Uuid, expense_id: i32 },
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/account")]
    Account {},
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
pub fn WebNavbar() -> Element {
    rsx! {
        Navbar {}
        Outlet::<Route> {}
    }
}
