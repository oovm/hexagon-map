use super::*;

/// A point in 3D stepped coordinate
/// s = - q - r
#[derive(Copy, Clone, Ord, PartialOrd, Eq, Hash, Serialize, Deserialize)]
pub struct AxialPoint {
    /// The horizontal axis
    /// - `T`: move to right
    /// - `F`: move to left
    pub q: isize,
    /// The right up axis
    /// - `T`: move to right up
    /// - `F`: move to left down
    pub r: isize,
}

impl AxialPoint {
    pub fn new(q: isize, r: isize) -> Self {
        Self { q, r }
    }
    pub fn go(&self, direction: Orientation) -> Self {
        <AxialPoint as Into<CubicPoint>>::into(*self).go(direction).into()
    }
}
