use crate::AxialPoint;
mod convert;
mod display;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Orientation {
    /// The horizontal axis, pointing to the right
    H(bool),
    P(bool),
    Q(bool),
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::H(true)
    }
}

impl Orientation {
    pub fn all() -> [Orientation; 6] {
        [
            Orientation::H(true),
            Orientation::P(true),
            Orientation::Q(true),
            Orientation::H(false),
            Orientation::P(false),
            Orientation::Q(false),
        ]
    }
    pub fn rotate(&self, clockwise: bool) -> Self {
        match clockwise {
            true => match self {
                Orientation::H(true) => Orientation::Q(false),
                Orientation::P(true) => Orientation::H(true),
                Orientation::Q(true) => Orientation::P(true),
                Orientation::H(false) => Orientation::Q(true),
                Orientation::P(false) => Orientation::H(false),
                Orientation::Q(false) => Orientation::P(false),
            },
            false => match self {
                Orientation::H(true) => Orientation::P(true),
                Orientation::P(true) => Orientation::Q(true),
                Orientation::Q(true) => Orientation::H(false),
                Orientation::H(false) => Orientation::P(false),
                Orientation::P(false) => Orientation::Q(false),
                Orientation::Q(false) => Orientation::H(true),
            },
        }
    }
}
impl Orientation {
    pub fn from_points(lhs: &AxialPoint, rhs: &AxialPoint) -> Option<Self> {
        let diff = rhs - lhs;
        match diff {
            AxialPoint { p: 1, q: 1 } => Some(Orientation::H(true)),
            AxialPoint { p: 1, q: 0 } => Some(Orientation::P(true)),
            AxialPoint { p: 0, q: 1 } => Some(Orientation::Q(true)),
            AxialPoint { p: -1, q: -1 } => Some(Orientation::H(false)),
            AxialPoint { p: -1, q: 0 } => Some(Orientation::P(false)),
            AxialPoint { p: 0, q: -1 } => Some(Orientation::Q(false)),
            _ => None,
        }
    }
    pub fn goto_points(&self, lhs: &AxialPoint) -> AxialPoint {
        match self {
            Orientation::H(true) => AxialPoint::new(lhs.p + 1, lhs.q + 1),
            Orientation::P(true) => AxialPoint::new(lhs.p + 1, lhs.q),
            Orientation::Q(true) => AxialPoint::new(lhs.p, lhs.q + 1),
            Orientation::H(false) => AxialPoint::new(lhs.p - 1, lhs.q - 1),
            Orientation::P(false) => AxialPoint::new(lhs.p - 1, lhs.q),
            Orientation::Q(false) => AxialPoint::new(lhs.p, lhs.q - 1),
        }
    }
}
