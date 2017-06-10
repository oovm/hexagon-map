use super::*;

/// A point in 3D stepped coordinate
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WPoint {
    /// Q-axis index, Z-axis index in cube coordinates
    pub x: isize,
    /// S-axis index, X-axis index in cube coordinates
    pub y: isize,
}

impl WPoint {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn go(&self, direction: Orientation) -> Self {
        <WPoint as Into<CubicPoint>>::into(*self).go(direction).into()
    }
}

impl From<CubicPoint> for WPoint {
    fn from(point: CubicPoint) -> Self {
        WPoint::new(point.p, point.q)
    }
}

impl Into<CubicPoint> for WPoint {
    fn into(self) -> CubicPoint {
        CubicPoint::new(self.x, self.y)
    }
}
