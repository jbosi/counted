use dioxus::logger::tracing;
use dioxus::prelude::*;
use shared::sse::EventSSE;
use web_sys::{wasm_bindgen::prelude::Closure, wasm_bindgen::JsCast, EventSource};

pub static SSE_EVENT_SOURCE: GlobalSignal<EventSource> =
    Signal::global(|| EventSource::new("").unwrap());

pub static SSE_USER_UPDATE: GlobalSignal<EventSSE> = Signal::global(|| EventSSE::None);
pub static SSE_PROJECT_UPDATE: GlobalSignal<EventSSE> = Signal::global(|| EventSSE::None);
pub static SSE_EXPENSE_UPDATE: GlobalSignal<EventSSE> = Signal::global(|| EventSSE::None);
pub static SSE_PAYMENT_UPDATE: GlobalSignal<EventSSE> = Signal::global(|| EventSSE::None);

pub fn init_global_signals_for_sse_events(
    events_sse: Vec<EventSSE>,
    global_signal: &'static GlobalSignal<EventSSE>,
) {
    let on_message = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MessageEvent| {
        if let Some(data) = event.data().as_string() {
            tracing::info!("SSE reçu: {:?}", data);
            global_signal.with_mut(|v| *v = data.parse().unwrap_or(EventSSE::None));
        }
    });

    events_sse.iter().for_each(|event| {
        SSE_EVENT_SOURCE()
            .add_event_listener_with_callback(
                &*event.to_string(),
                on_message.as_ref().unchecked_ref(),
            )
            .expect("couldn't set on message event listener");
    });

    SSE_EVENT_SOURCE().set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
}
