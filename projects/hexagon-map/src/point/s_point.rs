use super::*;

/// A point in 3D stepped coordinate
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CubePoint {
    /// Q-axis index, Z-axis index in cube coordinates
    pub h: isize,
    /// S-axis index, X-axis index in cube coordinates
    pub p: isize,
    /// R-axis index, Y-axis index in cube coordinates
    pub q: isize,
}

impl CubePoint {
    pub fn new(h: isize, p: isize, q: isize) -> Self {
        Self { h, p, q }
    }
    pub fn go(&self, direction: Orientation) -> Self {
        <CubePoint as Into<AxialPoint>>::into(*self).go(direction).into()
    }
}

impl From<AxialPoint> for CubePoint {
    fn from(point: AxialPoint) -> Self {
        CubePoint::new(point.p, -point.p - point.q, point.q)
    }
}

impl Into<AxialPoint> for CubePoint {
    fn into(self) -> AxialPoint {
        AxialPoint::new(self.h, self.q)
    }
}
