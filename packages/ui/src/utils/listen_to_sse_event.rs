use dioxus::logger::tracing;
use dioxus::prelude::*;
use shared::sse::EventSSE;
use web_sys::{wasm_bindgen::prelude::Closure, wasm_bindgen::JsCast, EventSource};

pub fn listen_to_sse_events(events_sse: Vec<EventSSE>, mut signal: Signal<String>) {
    use_effect(move || {
        let es = EventSource::new("/sse").expect("impossible d'ouvrir EventSource '/sse'");

        let on_message = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MessageEvent| {
            if let Some(data) = event.data().as_string() {
                tracing::info!("SSE reçu: {:?}", data);
                signal.set(data);
            }
        });

        events_sse.iter().for_each(|event| {
            es.add_event_listener_with_callback(
                &*event.to_string(),
                on_message.as_ref().unchecked_ref(),
            )
            .unwrap();
        });

        es.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();
    });
}
