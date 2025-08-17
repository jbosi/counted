use dioxus::prelude::*;
use std::{convert::Infallible, fmt, sync::Arc};

#[cfg(feature = "server")]
use axum::extract::State;
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

use once_cell::sync::Lazy;

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

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Debug, Clone, Copy)]
pub enum EventSSE {
    UserCreated,
    UserDeleted,
    UserModified,
    ProjectCreated,
    ProjectDeleted,
    ProjectModified,
    ExpenseCreated,
    ExpenseDeleted,
    ExpenseModified,
    PaymentCreated,
    PaymentDeleted,
    PaymentModified,
}

impl fmt::Display for EventSSE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventSSE::UserCreated => write!(f, "UserCreated"),
            EventSSE::UserDeleted => write!(f, "UserDeleted"),
            EventSSE::UserModified => write!(f, "UserModified"),
            EventSSE::ProjectCreated => write!(f, "ProjectCreated"),
            EventSSE::ProjectDeleted => write!(f, "ProjectDeleted"),
            EventSSE::ProjectModified => write!(f, "ProjectModified"),
            EventSSE::ExpenseCreated => write!(f, "ExpenseCreated"),
            EventSSE::ExpenseDeleted => write!(f, "ExpenseDeleted"),
            EventSSE::ExpenseModified => write!(f, "ExpenseModified"),
            EventSSE::PaymentCreated => write!(f, "PaymentCreated"),
            EventSSE::PaymentDeleted => write!(f, "PaymentDeleted"),
            EventSSE::PaymentModified => write!(f, "PaymentModified"),
        }
    }
}
