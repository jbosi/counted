use dioxus::prelude::*;
use ui::route::Route;

use shared::sse::EventSSE;

#[cfg(feature = "server")]
use api::sse::sse_handler;
#[cfg(feature = "server")]
use api::sse::{AppState, Broadcaster};
#[cfg(feature = "server")]
use axum::extract::Request;
#[cfg(feature = "server")]
use axum::middleware::Next;
#[cfg(feature = "server")]
use axum::routing::{get, post};
#[cfg(feature = "server")]
use axum::{middleware, response::IntoResponse, ServiceExt};
#[cfg(feature = "server")]
use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::prelude::Closure;
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::EventSource;

#[cfg(feature = "server")]
use axum::extract::Extension;
#[cfg(feature = "server")]
use axum::extract::FromRequestParts;
#[cfg(feature = "server")]
use http::request::Parts;
// #[cfg(feature = "server")]
use serde::{Deserialize, Serialize};
use tracing::info;

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
        tokio::runtime::Runtime::new().unwrap().block_on(async move {
            // Session layer (in-memory store for demo; replace with persistent store in prod)
            let store = MemoryStore::default();
            let session_layer = SessionManagerLayer::new(store)
                .with_name("counted.session")
                .with_secure(false)
                .with_expiry(Expiry::OnSessionEnd);

            // Routes
            let app_routes = axum::Router::new()
                .route("/login", post(login))
                .route("/logout", post(logout))
                // protect the SSE route with RequireAuth
                .route("/sse", get(sse_handler).route_layer(middleware::from_fn(require_auth)))
                .layer(session_layer)
                .serve_dioxus_application(ServeConfig::new().unwrap(), app);

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
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        // document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        main {
            class: "min-h-screen flex flex-col items-center",
            link {
                rel: "stylesheet",
                href: "../assets/tailwind.css",
            },
            link {
                rel: "stylesheet",
                href: "https://cdn.jsdelivr.net/npm/daisyui@5",
            },
            Router::<Route> {}
        }
    }
}

// --- Session auth helpers (server only) ---

#[cfg(feature = "server")]
async fn login(Extension(session): Extension<Session>) -> impl IntoResponse {
    // In a real app, verify credentials first; here we just set a user id flag
    let _ = session.insert("user_id", "demo").await;
    let _ = session.save().await;
    (axum::http::StatusCode::OK, "logged in")
}

#[cfg(feature = "server")]
async fn logout(Extension(session): Extension<Session>) -> impl IntoResponse {
    let _ = session.flush().await; // remove from store and clear cookie
    (axum::http::StatusCode::OK, "logged out")
}

#[cfg(feature = "server")]
async fn require_auth(req: Request, next: Next) -> impl IntoResponse {
    let session = req.extensions().get::<Session>().cloned();
    let authenticated = if let Some(session) = session {
        use tracing::info;

        // Ensure the session record is hydrated from the store
        if let Err(err) = session.load().await {
            info!("Session load error: {:?}", err);
        }
        info!("Session found: {:?}", session);
        match session.get::<String>("user_id").await {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(err) => {
                info!("Session get error: {:?}", err);
                false
            }
        }
    } else {
        false
    };
    if authenticated {
        next.run(req).await
    } else {
        (axum::http::StatusCode::UNAUTHORIZED, "unauthorized").into_response()
    }
}

#[server]
pub async fn timeout() -> Result<(), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Ok(())
}

// *******************************************

const AUTHENTICATION_KEY: &str = "authentication";
#[derive(Default, Deserialize, Serialize, Debug)]
struct Authentication {
    is_anonymous: bool,
}

#[cfg(feature = "server")]
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(req, state).await?;
        let authentication: Authentication =
            session.get(AUTHENTICATION_KEY).await.unwrap().unwrap_or_default();
        info!("Authentication found: {:?}", authentication);
        session.insert(AUTHENTICATION_KEY, Authentication { is_anonymous: false }).await.unwrap();
        Ok(authentication)
    }
}
