use serde::{Serialize, Deserialize};

pub mod s_point;
pub mod w_point;
pub mod h_point;

/// A point in axial coordinates, standard form of a hexagon map
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AxialPoint {
    pub q: isize,
    pub r: isize,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum HDirection {
    /// - `S-true means right`
    /// - `S-false means left`
    S(bool),
    R(bool),
    Q(bool),
}

impl AxialPoint {
    pub fn new(q: isize, r: isize) -> Self {
        Self { q, r }
    }
    pub fn from_pixel(x: f64, y: f64, radius: f64) -> Self {
        let q = (x * 3.0f64.sqrt() / 3.0 - y / 3.0) / radius;
        let r = y * 2.0 / 3.0 / radius;
        Self::new(q.round() as isize, r.round() as isize)
    }
    pub fn go(&self, direction: HDirection) -> Self {
        match direction {
            HDirection::S(true) => Self::new(self.q + 1, self.r - 1),
            HDirection::S(false) => Self::new(self.q - 1, self.r + 1),
            HDirection::R(true) => Self::new(self.q + 1, self.r),
            HDirection::R(false) => Self::new(self.q - 1, self.r),
            HDirection::Q(true) => Self::new(self.q, self.r - 1),
            HDirection::Q(false) => Self::new(self.q, self.r + 1),
        }
    }
}




