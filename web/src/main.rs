use dioxus::prelude::*;
use ui::route::Route;

#[cfg(feature = "server")]
use api::sse::sse_handler;
#[cfg(feature = "server")]
use axum::routing::get;
#[cfg(feature = "server")]
use axum::Router;
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::prelude::Closure;
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::JsCast;
#[cfg(feature = "web")]
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
            let app_routes = Router::new()
                .serve_dioxus_application(ServeConfig::new().unwrap(), app)
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
    // Initialise la connexion SSE côté client (WebAssembly) depuis un composant
    // pour respecter les règles des hooks Dioxus.
    #[cfg(feature = "web")]
    {
        let mut es_handle = use_signal::<Option<EventSource>>(|| None);

        use_effect(move || {
            // Utilise une URL relative pour rester sur la même origine que le serveur Dioxus
            let es = EventSource::new("/sse").expect("impossible d'ouvrir EventSource '/sse'");

            // Callback à chaque message
            let on_message = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MessageEvent| {
                if let Some(data) = event.data().as_string() {
                    tracing::info!("SSE reçu: {:?}", data);
                }
            });

            // Attache la callback et garde sa closure vivante
            es.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
            on_message.forget();

            // Conserve l'EventSource pour éviter sa libération prématurée
            es_handle.set(Some(es));
        });
    }

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
