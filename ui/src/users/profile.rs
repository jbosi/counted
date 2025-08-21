use crate::common::{AppHeader, Avatar};
use crate::expenses::{ExpenseList, ExpensesUserSection, SummaryCard};
use crate::modals::AddExpenseModal;
use crate::route::Route;
use crate::utils::listen_to_sse_events;
use api::expenses::get_expenses_by_project_id;
use api::projects::get_project_by_id;
use api::users::{add_registered_user, add_user, get_users_by_project_id};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use shared::sse::EventSSE::{ExpenseCreated, ExpenseDeleted, ExpenseModified};
use shared::view_models::users_project_view_model::UsersProject;
use shared::{CreatableProject, CreatableRegisteredUser, CreatableUser, Expense, UserDto};
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct ProfileComponentProps {}

#[component]
pub fn ProfileComponent(props: ProfileComponentProps) -> Element {
    let mut user_lastname: Signal<String> = use_signal(|| String::new());
    let mut user_firstname: Signal<String> = use_signal(|| String::new());
    let mut user_email: Signal<String> = use_signal(|| String::new());
    let mut user_password: Signal<String> = use_signal(|| String::new());
    let mut user_phone_number: Signal<String> = use_signal(|| String::new());

    rsx! {
        div {
            class: "container overflow-auto app-container bg-base-200 p-4 max-w-md rounded-xl flex flex-col",
            AppHeader { title: "Profile", back_button_route: Route::Projects {} }

            div {
                class: "mt-6 flex flex-col items-center",
                Avatar { initials: "JB", size: 32, text_size: "2xl" },

                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Nom" }
                    input {
                        name: "project_description",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| user_lastname.set(event.value()),
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Prénom" }
                    input {
                        name: "project_name",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| user_firstname.set(event.value()),
                    }
                }
                 fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Téléphone" }
                    input {
                        name: "project_name",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| user_phone_number.set(event.value()),
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Email" }
                    input {
                        name: "project_name",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| user_email.set(event.value()),
                    }
                }
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Mot de passe" }
                    input {
                        name: "project_name",
                        r#type: "text",
                        class: "input",
                        oninput: move |event| user_password.set(event.value()),
                    }
                }
                button {
                    r#type: "submit",
                    onclick: move |_| {
                        spawn(async move {
                            let creatable_user: CreatableRegisteredUser = CreatableRegisteredUser {
                                firstname: user_firstname(),
                                lastname: user_lastname(),
                                email: user_email(),
                                password: user_password(),
                                phone_number: user_phone_number(),
                            };
                            add_registered_user(creatable_user).await.expect("Failed to add new registred user");
                        });
                    },
                    "Enregistrer"
                }
            }
        }
    }
}
