use super::*;

/// A point in 3D stepped coordinate
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SPoint {
    /// Q-axis index, Z-axis index in cube coordinates
    pub q: isize,
    /// S-axis index, X-axis index in cube coordinates
    pub s: isize,
    /// R-axis index, Y-axis index in cube coordinates
    pub r: isize,
}

/// A point in width first coordinates
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct WPoint {
    pub x: isize,
    pub y: isize,
}

/// A point in height first coordinates
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HPoint {
    pub y: isize,
    pub x: isize,
}


impl WPoint {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn go(&self, direction: HDirection) -> Self {
        self.as_axial().go(direction).as_w_point()
    }
    pub fn as_axial(&self) -> AxialPoint {
        AxialPoint::new(self.x, self.y)
    }
}

impl HPoint {
    pub fn new(x: isize, y: isize) -> Self {
        Self { y, x }
    }
    pub fn go(&self, direction: HDirection) -> Self {
        self.as_axial().go(direction).as_h_point()
    }
    pub fn as_axial(&self) -> AxialPoint {
        AxialPoint::new(self.x, self.y)
    }
}

impl SPoint {
    pub fn new(q: isize, s: isize, r: isize) -> Self {
        Self { q, s, r }
    }
    pub fn go(&self, direction: HDirection) -> Self {
        self.as_axial().go(direction).as_s_point()
    }
    pub fn as_axial(&self) -> AxialPoint {
        // q + r + s = 0
        AxialPoint::new(self.q, self.r)
    }
}

impl From<AxialPoint> for SPoint {
    fn from(point: AxialPoint) -> Self {
        SPoint::new(point.q, -point.q - point.r, point.r)
    }
}

impl Into<AxialPoint> for SPoint {
    fn into(self) -> AxialPoint {
        AxialPoint::new(self.q, self.r)
    }
}

impl From<AxialPoint> for WPoint {
    fn from(point: AxialPoint) -> Self {
        point.as_w_point()
    }
}

impl From<AxialPoint> for HPoint {
    fn from(point: AxialPoint) -> Self {
        point.as_h_point()
    }
}

impl AxialPoint {
    pub fn as_s_point(&self) -> SPoint {
        // q + r + s = 0
        SPoint::new(self.q, -self.q - self.r, self.r)
    }
    pub fn as_w_point(&self) -> WPoint {
        WPoint::new(self.q, self.r)
    }
    pub fn as_h_point(&self) -> HPoint {
        HPoint::new(self.r, self.q)
    }
}