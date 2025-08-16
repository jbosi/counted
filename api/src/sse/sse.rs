use dioxus::prelude::*;
use std::{convert::Infallible, time::Duration};

#[cfg(feature = "server")]
use axum::response::sse::{Event, KeepAlive, Sse};
#[cfg(feature = "server")]
use futures_util::stream::{self, Stream};
#[cfg(feature = "server")]
use tokio_stream::StreamExt;

#[cfg(feature = "server")]
pub async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream =
        stream::repeat_with(|| Event::default().data("")).map(Ok).throttle(Duration::from_secs(5));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
