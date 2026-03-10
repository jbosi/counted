use api::auth::auth_controller::login;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, LoginPayload};

#[component]
pub fn Login() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);

    let nav = use_navigator();
    let mut auth_ctx = use_context::<Signal<Option<Account>>>();

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let email_val = email();
        let password_val = password();

        async move {
            loading.set(true);
            error_msg.set(None);

            match login(Json(LoginPayload { email: email_val, password: password_val })).await {
                Ok(account) => {
                    auth_ctx.set(Some(account));
                    nav.push("/");
                }
                Err(e) => {
                    error_msg.set(Some(e.to_string()));
                    loading.set(false);
                }
            }
        }
    };

    rsx! {
        div { class: "container p-4 max-w-sm mx-auto flex flex-col gap-6 mt-16",
            h1 { class: "text-3xl font-light text-center", "Sign in" }

            if let Some(err) = error_msg() {
                div { class: "alert alert-error", "{err}" }
            }

            form { class: "flex flex-col gap-4", onsubmit: on_submit,
                label { class: "form-control",
                    span { class: "label-text mb-1", "Email" }
                    input {
                        class: "input input-bordered",
                        r#type: "email",
                        required: true,
                        value: "{email}",
                        oninput: move |e| email.set(e.value()),
                    }
                }
                label { class: "form-control",
                    span { class: "label-text mb-1", "Password" }
                    input {
                        class: "input input-bordered",
                        r#type: "password",
                        required: true,
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                    }
                }
                button {
                    class: "btn btn-primary",
                    r#type: "submit",
                    disabled: loading(),
                    if loading() { "Signing in…" } else { "Sign in" }
                }
            }

            p { class: "text-center text-sm",
                "No account? "
                a { class: "link link-primary", href: "/register", "Register" }
            }
        }
    }
}
