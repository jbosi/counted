use dioxus::prelude::*;
use ui::route::Route;

use shared::sse::EventSSE;

use api::authentication::User;
#[cfg(feature = "server")]
use api::db::get_db;
#[cfg(feature = "server")]
use api::sse::sse_handler;
#[cfg(feature = "server")]
use api::sse::{AppState, Broadcaster};
#[cfg(feature = "server")]
use axum::routing::get;
#[cfg(feature = "server")]
use axum::{Router, ServiceExt};
#[cfg(feature = "server")]
use axum_session::{Session, SessionConfig, SessionLayer, SessionStore};
#[cfg(feature = "server")]
use axum_session_auth::{AuthConfig, AuthSessionLayer};
#[cfg(feature = "server")]
use axum_session_sqlx::SessionPgPool;
#[cfg(feature = "server")]
use sqlx::PgPool;
#[cfg(feature = "server")]
use sqlx::{Pool, Postgres};
use web_sys::wasm_bindgen::prelude::Closure;
use web_sys::wasm_bindgen::JsCast;
use web_sys::EventSource;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Set the logger ahead of time since we don't use `dioxus::launch` on the server
    dioxus::logger::initialize_default();

    #[cfg(feature = "web")]
    // Hydrate the application on the client
    LaunchBuilder::web().launch(app);

    #[cfg(feature = "server")]
    {
        use axum::routing::*;
        tokio::runtime::Runtime::new().unwrap().block_on(async move {
            let pool: Pool<Postgres> = get_db().await;

            let session_config = SessionConfig::default().with_table_name("test_table");
            let auth_config = AuthConfig::<i32>::default().with_anonymous_user_id(Some(1));
            let session_store =
                SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config)
                    .await
                    .unwrap();

            let app_routes = Router::new()
                .serve_dioxus_application(ServeConfig::new().unwrap(), app)
                .layer(
                    AuthSessionLayer::<User, i32, SessionPgPool, PgPool>::new(Some(pool))
                        .with_config(auth_config),
                )
                .layer(SessionLayer::new(session_store))
                .route("/sse", get(sse_handler));

            // serve the app using the address passed by the CLI
            let addr = dioxus::cli_config::fullstack_address_or_localhost();
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

            axum::serve(listener, app_routes.into_make_service()).await.unwrap();
        });
    }
}

#[component]
fn app() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        main { class: "min-h-screen flex flex-col items-center",
            link {
                rel: "stylesheet",
                href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css",
            }
            Router::<Route> {}
        }
    }
}
