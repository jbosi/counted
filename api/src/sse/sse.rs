#[cfg(feature = "server")]
use axum::{
    extract::State,
    response::{sse::Event, Sse},
    routing::get,
    Router,
};
use dioxus::prelude::*;
// use futures::stream::Stream;
use std::{convert::Infallible, time::Duration};
#[cfg(feature = "server")]
use tokio_stream::{Stream, StreamExt};

// #[cfg(feature = "server")]
// pub fn sse_routes() -> Router {
//     Router::new().route("/events", get(sse_handler))
// }
//
// #[server]
// async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
//     let stream = tokio_stream::iter(0..).then(|count| async move {
//         tokio::time::sleep(Duration::from_secs(1)).await;
//         Ok(Event::default().data(format!("count: {}", count)))
//     });
//
//     Sse::new(stream)
// }
