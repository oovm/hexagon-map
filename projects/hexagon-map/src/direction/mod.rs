use crate::AxialPoint;
mod convert;
mod display;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Orientation {
    /// - `S-true means right`
    /// - `S-false means left`
    S(bool),
    R(bool),
    Q(bool),
}

impl Orientation {
    pub fn all() -> [Orientation; 6] {
        [
            Orientation::S(true),
            Orientation::S(false),
            Orientation::R(true),
            Orientation::R(false),
            Orientation::Q(true),
            Orientation::Q(false),
        ]
    }
    pub fn rotate(&self, clockwise: bool) -> Self {
        match clockwise {
            true => match self {
                Orientation::S(true) => Orientation::R(true),
                Orientation::S(false) => Orientation::Q(true),
                Orientation::R(true) => Orientation::Q(false),
                Orientation::R(false) => Orientation::S(false),
                Orientation::Q(true) => Orientation::S(true),
                Orientation::Q(false) => Orientation::R(false),
            },
            false => match self {
                Orientation::S(true) => Orientation::Q(false),
                Orientation::S(false) => Orientation::R(false),
                Orientation::R(true) => Orientation::S(true),
                Orientation::R(false) => Orientation::Q(true),
                Orientation::Q(true) => Orientation::R(true),
                Orientation::Q(false) => Orientation::S(false),
            },
        }
    }
}
impl Orientation {
    pub fn from_points(lhs: &AxialPoint, rhs: &AxialPoint) -> Option<Self> {
        let q = rhs.q - lhs.q;
        let r = rhs.r - lhs.r;
        match (q, r) {
            (1, -1) => Some(Orientation::S(true)),
            (-1, 1) => Some(Orientation::S(false)),
            (1, 0) => Some(Orientation::R(true)),
            (-1, 0) => Some(Orientation::R(false)),
            (0, -1) => Some(Orientation::Q(true)),
            (0, 1) => Some(Orientation::Q(false)),
            _ => None,
        }
    }
    pub fn goto_points(&self, lhs: &AxialPoint) -> AxialPoint {
        match self {
            Orientation::S(true) => AxialPoint::new(lhs.q + 1, lhs.r - 1),
            Orientation::S(false) => AxialPoint::new(lhs.q - 1, lhs.r + 1),
            Orientation::R(true) => AxialPoint::new(lhs.q + 1, lhs.r),
            Orientation::R(false) => AxialPoint::new(lhs.q - 1, lhs.r),
            Orientation::Q(true) => AxialPoint::new(lhs.q, lhs.r - 1),
            Orientation::Q(false) => AxialPoint::new(lhs.q, lhs.r + 1),
        }
    }
}
