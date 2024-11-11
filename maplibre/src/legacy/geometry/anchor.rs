//! Translated from https://github.com/maplibre/maplibre-native/blob/4add9ea/src/mbgl/geometry/anchor.hpp

use crate::{euclid::Point2D, legacy::TileSpace};

/// maplibre/maplibre-native#4add9ea original name: Anchor
#[derive(Clone, Copy)]
pub struct Anchor {
    pub point: Point2D<f64, TileSpace>,
    pub angle: f64,
    pub segment: Option<usize>,
}

/// maplibre/maplibre-native#4add9ea original name: Anchors
pub type Anchors = Vec<Anchor>;