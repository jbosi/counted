use api::auth::auth_controller::logout;
use dioxus::prelude::*;
use shared::Account;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar(children: Element) -> Element {
    let mut auth_ctx = use_context::<Signal<Option<Account>>>();
    let nav = use_navigator();

    let on_logout = move |_| {
        async move {
            let _ = logout().await;
            auth_ctx.set(None);
            nav.push("/login");
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { id: "navbar",
            {children}

            div { class: "navbar-auth",
                match auth_ctx() {
                    Some(account) => rsx! {
                        span { class: "navbar-display-name", "{account.display_name}" }
                        button { class: "btn btn-ghost btn-sm", onclick: on_logout, "Logout" }
                    },
                    None => rsx! {
                        a { class: "btn btn-ghost btn-sm", href: "/login", "Sign in" }
                        a { class: "btn btn-primary btn-sm", href: "/register", "Register" }
                    },
                }
            }
        }
    }
}
