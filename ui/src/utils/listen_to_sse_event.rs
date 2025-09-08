use dioxus::logger::tracing;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use eventsource_client::{Client, Error, SSE::Event};
use futures_util::{FutureExt, Stream, StreamExt, TryStreamExt};
use shared::sse::EventSSE;
use std::pin::Pin;
use std::time::Duration;
use web_sys::{wasm_bindgen::prelude::Closure, wasm_bindgen::JsCast, EventSource};

pub static SSE_EVENT_SOURCE: GlobalSignal<EventSource> =
    Signal::global(|| EventSource::new("").unwrap());

pub static SSE_USER_UPDATE: GlobalSignal<EventSSE> = Signal::global(|| EventSSE::None);
pub static SSE_PROJECT_UPDATE: GlobalSignal<EventSSE> = Signal::global(|| EventSSE::None);
pub static SSE_EXPENSE_UPDATE: GlobalSignal<EventSSE> = Signal::global(|| EventSSE::None);
pub static SSE_PAYMENT_UPDATE: GlobalSignal<EventSSE> = Signal::global(|| EventSSE::None);

#[cfg(feature = "server")]
pub fn init_global_signals_for_sse_events(// events_sse: Vec<EventSSE>,
    // global_signal: &'static GlobalSignal<EventSSE>,
) -> Pin<Box<dyn Stream<Item = Result<String, Error>>>> {
    // let on_message = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MessageEvent| {
    //     if let Some(data) = event.data().as_string() {
    //         tracing::info!("SSE reçu: {:?}", data);
    //         global_signal.with_mut(|v| *v = data.parse().unwrap_or(EventSSE::None));
    //     }
    // });
    //
    // events_sse.iter().for_each(|event| {
    //     SSE_EVENT_SOURCE()
    //         .add_event_listener_with_callback(
    //             &*event.to_string(),
    //             on_message.as_ref().unchecked_ref(),
    //         )
    //         .expect("couldn't set on message event listener");
    // });
    //
    // SSE_EVENT_SOURCE().set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    // on_message.forget();

    let sse_client = eventsource_client::ClientBuilder::for_url("http://127.0.0.1:8080/sse")
        .unwrap()
        .reconnect(
            eventsource_client::ReconnectOptions::reconnect(true)
                .retry_initial(false)
                .delay(Duration::from_secs(1))
                .backoff_factor(2)
                .delay_max(Duration::from_secs(20))
                .build(),
        )
        .build();

    let mut sse_stream = sse_client.stream();

    sse_stream.next().now_or_never(); // Poll once to start connecting immediately

    let sse_message_stream = sse_stream
        .try_filter_map(|response| async move {
            if let Event(event) = response {
                // if let Some(data) = event.data {
                tracing::info!("SSE reçu: {:?}", event.data);
                event.data;
                // }
            }
            Ok(None)
        })
        .into_stream();

    Box::pin(sse_message_stream)
}
