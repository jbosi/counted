use crate::auth::{account::Account, login::Login, register::Register};
use crate::payments::PaymentPage;
use crate::project_details::ProjectDetails;
use crate::projects::ProjectsList;
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
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
