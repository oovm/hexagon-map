use super::*;

/// A point in axial coordinates, standard form of a hexagon map
#[derive(Copy, Clone, Ord, PartialOrd, Eq, Hash, Serialize, Deserialize)]
pub struct CubicPoint {
    pub p: isize,
    pub q: isize,
}

impl CubicPoint {
    /// Create a new point in axial coordinates
    pub fn new(p: isize, q: isize) -> Self {
        Self { p, q }
    }
    /// Create a new point in axial coordinates from pixel coordinates
    pub fn from_pixel(x: f64, y: f64, radius: f64) -> Self {
        let q = (x * 3.0f64.sqrt() / 3.0 - y / 3.0) / radius;
        let r = y * 2.0 / 3.0 / radius;
        Self::new(q.round() as isize, r.round() as isize)
    }
}
