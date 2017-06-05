use super::*;

/// A point in 3D stepped coordinate
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CubePoint {
    /// Q-axis index, Z-axis index in cube coordinates
    pub q: isize,
    /// S-axis index, X-axis index in cube coordinates
    pub s: isize,
    /// R-axis index, Y-axis index in cube coordinates
    pub r: isize,
}

impl CubePoint {
    pub fn new(q: isize, s: isize, r: isize) -> Self {
        Self { q, s, r }
    }
    pub fn go(&self, direction: Orientation) -> Self {
        <CubePoint as Into<AxialPoint>>::into(*self).go(direction).into()
    }
}

impl From<AxialPoint> for CubePoint {
    fn from(point: AxialPoint) -> Self {
        CubePoint::new(point.q, -point.q - point.r, point.r)
    }
}

impl Into<AxialPoint> for CubePoint {
    fn into(self) -> AxialPoint {
        AxialPoint::new(self.q, self.r)
    }
}
