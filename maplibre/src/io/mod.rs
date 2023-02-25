//! Handles IO related processing as well as multithreading.

pub use geozero::mvt::tile::Layer as RawLayer;

pub mod apc;
pub mod geometry_index;
pub mod http;
pub mod scheduler;
pub mod source;
#[cfg(feature = "embed-static-tiles")]
pub mod static_tile_fetcher;
