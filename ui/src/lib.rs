//! This crate contains all shared UI for the workspace.

mod navbar;
pub use navbar::Navbar;

mod avatar;
pub use avatar::Avatar;

mod back_button_arrow;
pub use back_button_arrow::BackButtonArrow;

mod app_header;
pub use app_header::AppHeader;

pub mod route;
pub mod expenses;
pub mod projects;
pub mod modals;
pub mod payments;
