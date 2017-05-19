use serde::{Serialize, Deserialize};


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AxialPoint {
    /// Q-axis index, Z-axis index in cube coordinates
    pub q: isize,
    /// R-axis index, Y-axis index in cube coordinates
    pub r: isize,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CPoint {
    /// Q-axis index, Z-axis index in cube coordinates
    pub q: isize,
    /// S-axis index, X-axis index in cube coordinates
    pub s: isize,
    /// R-axis index, Y-axis index in cube coordinates
    pub r: isize,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WPoint {
    pub x: isize,
    pub y: isize,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HPoint {
    pub x: isize,
    pub y: isize,
}



impl AxialPoint {
    pub fn new(q: isize, r: isize) -> Self {
        Self { q, r }
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
    pub fn as_hex(&self) -> CPoint {
        // q + r + s = 0
        CPoint::new(self.q, -self.q - self.r, self.r)
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum HDirection {
    S(bool),
    R(bool),
    Q(bool),
}

impl CPoint {
    pub fn new(q: isize, s: isize, r: isize) -> Self {
        Self { q, s, r }
    }
    pub fn go(&self, direction: HDirection) -> Self {
        self.as_axial().go(direction).as_hex()
    }
    pub fn as_axial(&self) -> AxialPoint {
        // q + r + s = 0
        AxialPoint::new(self.q, self.r)
    }
}