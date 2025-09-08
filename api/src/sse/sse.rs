use dioxus::prelude::*;
use once_cell::sync::Lazy;
use std::{convert::Infallible, sync::Arc};

#[cfg(feature = "server")]
use axum::response::sse::{Event, KeepAlive, Sse};
use dioxus::logger::tracing::info;
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
    clients: Arc<Mutex<Vec<mpsc::Sender<Event>>>>,
}

#[cfg(feature = "server")]
impl Broadcaster {
    pub fn new() -> Self {
        Self { clients: Arc::new(Mutex::new(Vec::new())) }
    }

    /// Register a new client and return a stream of events for it
    pub async fn new_client(&self) -> impl Stream<Item = Result<Event, Infallible>> {
        let (tx, rx) = mpsc::channel(10);

        self.clients.lock().await.push(tx);

        ReceiverStream::new(rx).map(Ok)
    }

    /// Send an event to all connected clients
    pub async fn broadcast(&self, event: Event) {
        let mut clients = self.clients.lock().await;

        // retain only connected clients
        clients.retain(|client| client.try_send(event.clone()).is_ok());
    }
}

/// SSE handler: subscribes a client to the broadcaster
#[cfg(feature = "server")]
pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = BROADCASTER.new_client().await;
    info!("ok");
    Sse::new(stream).keep_alive(KeepAlive::default())
}
