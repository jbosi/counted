use dioxus::fullstack::ServerEvents;
use dioxus::prelude::*;
use once_cell::sync::Lazy;
use shared::sse::EventSSE;
use std::{convert::Infallible, sync::Arc};
use crate::sse::sse::dioxus_fullstack::SseTx;

#[cfg(feature = "server")]
use axum::response::sse::{Event, KeepAlive, Sse};
#[cfg(feature = "server")]
use futures_util::stream::Stream;
#[cfg(feature = "server")]
use tokio::sync::{mpsc, Mutex};
#[cfg(feature = "server")]
use tokio_stream::wrappers::ReceiverStream;
#[cfg(feature = "server")]
use tokio_stream::StreamExt;

#[cfg(feature = "server")]
pub static BROADCASTER: Lazy<Arc<Broadcaster>> = Lazy::new(|| Arc::new(Broadcaster::new()));

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct AppState {
    pub broadcaster: Broadcaster,
}

/// Manages all connected SSE clients
#[cfg(feature = "server")]
#[derive(Clone)]
pub struct Broadcaster {
    clients: Arc<Mutex<Vec<SseTx<EventSSE>>>>,
}

#[cfg(feature = "server")]
impl Broadcaster {
    pub fn new() -> Self {
        Self { clients: Arc::new(Mutex::new(Vec::new())) }
    }

    /// Register a new client and return a stream of events for it
    pub async fn new_client(&self) -> Result<ServerEvents<EventSSE>> {
        use std::time::Duration;

        Ok(ServerEvents::new(move |mut tx| async move {
            self.clients.lock().await.push(tx);

            loop {
                // Create our serializable message
                let msg = EventSSE::None;

                // Send the message to the client. If it errors, the client has disconnected
                // if tx.send(msg).await.is_err() {
                //     // client disconnected, do some cleanup
                //     break;
                // }

                // Poll some data source here, subscribe to changes, maybe call an LLM?
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }))
    }

    /// Send an event to all connected clients
    pub async fn broadcast(&self, event: Event) {
        let mut clients: Arc<Mutex<Vec<SseTx<EventSSE>>>> = self.clients.lock().await;

        // retain only connected clients
        clients.retain(|client| client.send(event.clone()).is_ok());
    }
}

/// SSE handler: subscribes a client to the broadcaster
// #[cfg(feature = "server")]
// pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
//     let stream = BROADCASTER.new_client().await;
//
//     Sse::new(stream).keep_alive(KeepAlive::default())
// }

/// Our SSE endpoint, when called, will return the ServerEvents handle which streams events to the client.
/// On the client, we can interact with this stream object to get new events as they arrive.
#[get("/sse")]
async fn listen_for_changes() -> Result<ServerEvents<EventSSE>> {
    BROADCASTER.new_client().await
    // Sse::new(stream).keep_alive(KeepAlive::default())
}
