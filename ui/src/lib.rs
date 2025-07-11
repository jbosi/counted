//! This crate contains all shared UI for the workspace.


mod navbar;
pub use navbar::Navbar;


mod avatar;
pub use avatar::Avatar;

mod add_user_modal;
pub use add_user_modal::AddUserModal;

mod back_button_arrow;
pub use back_button_arrow::BackButtonArrow;

mod add_project_modal;
pub use add_project_modal::AddProjectModal;
