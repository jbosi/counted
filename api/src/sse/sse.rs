#[cfg(feature = "server")]
use axum::{
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
    Router,
};
// use axum::{
//     extract::State,
//     response::{sse::Event, Sse},
//     routing::get,
//     Router,
// };
use dioxus::prelude::*;
// use futures::stream::Stream;
#[cfg(feature = "server")]
use futures_util::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};
#[cfg(feature = "server")]
use tokio_stream::StreamExt;
// #[cfg(feature = "server")]
// use tokio_stream::{Stream, StreamExt};
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

#[cfg(feature = "server")]
pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(5));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
