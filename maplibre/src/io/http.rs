//! HTTP client.

use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("failed to fetch from http")]
pub struct HttpError(#[source] pub Box<dyn std::error::Error>);

/// A closure that returns a HTTP client.
pub type HTTPClientFactory<HC> = dyn Fn() -> HC;

/// On the web platform futures are not thread-safe (i.e. not Send). This means we need to tell
/// async_trait that these bounds should not be placed on the async trait:
/// [https://github.com/dtolnay/async-trait/blob/b70720c4c1cc0d810b7446efda44f81310ee7bf2/README.md#non-threadsafe-futures](https://github.com/dtolnay/async-trait/blob/b70720c4c1cc0d810b7446efda44f81310ee7bf2/README.md#non-threadsafe-futures)
///
/// Users of this library can decide whether futures from the HTTPClient are thread-safe or not via
/// the future "thread-safe-futures". Tokio futures are thread-safe.
#[cfg_attr(not(feature = "thread-safe-futures"), async_trait(?Send))]
#[cfg_attr(feature = "thread-safe-futures", async_trait)]
pub trait HttpClient: Clone + Sync + Send + 'static {
    async fn fetch(&self, url: &str) -> Result<Vec<u8>, HttpError>;
}
