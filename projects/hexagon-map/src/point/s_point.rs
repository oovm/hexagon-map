use super::*;

/// A point in 3D stepped coordinate
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CubePoint {
    /// The horizontal axis
    /// - `T`: move to right
    /// - `F`: move to left
    pub h: isize,
    /// The right up axis
    /// - `T`: move to right up
    /// - `F`: move to left down
    pub p: isize,
    /// The left up axis
    /// - `T`: move to left up
    /// - `F`: move to right down
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
        CubePoint::new(point.p - point.q, point.p, point.q)
    }
}

impl Into<AxialPoint> for CubePoint {
    fn into(self) -> AxialPoint {
        AxialPoint::new(self.p, self.q)
    }
}
