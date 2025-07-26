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

mod add_expense_modal;
pub use add_expense_modal::AddExpenseModal;

mod app_header;
pub use app_header::AppHeader;
pub mod route;

mod expenses;
pub use expenses::Expenses;

mod payments;
pub use payments::Payments;

mod project;
pub use project::Project;

mod projects;
pub use projects::Projects;

pub use expenses::expense_list::ExpenseList;
pub use expenses::expense_item::ExpenseItem;
pub use expenses::summary_card::SummaryCard;
pub use expenses::expenses_user_section::ExpensesUserSection;