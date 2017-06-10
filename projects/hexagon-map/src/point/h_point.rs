use super::*;

/// A point in 3D stepped coordinate
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HPoint {
    /// S-axis index, X-axis index in cube coordinates
    pub y: isize,
    /// Q-axis index, Z-axis index in cube coordinates
    pub x: isize,
}

impl HPoint {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn go(&self, direction: Orientation) -> Self {
        <HPoint as Into<CubicPoint>>::into(*self).go(direction).into()
    }
}
