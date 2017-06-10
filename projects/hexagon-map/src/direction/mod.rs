use crate::CubePoint;
mod convert;
mod display;
use serde::{Deserialize, Serialize};

/// Orientation in the HPQ coordinate system
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Orientation {
    /// The horizontal axis
    /// - `T`: move to right
    /// - `F`: move to left
    H(bool),
    /// The right up axis
    /// - `T`: move to right up
    /// - `F`: move to left down
    P(bool),
    /// The left up axis
    /// - `T`: move to left up
    /// - `F`: move to right down
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
    pub fn from_points(lhs: &CubePoint, rhs: &CubePoint) -> Option<Self> {
        let dp = rhs.p - lhs.p;
        let dq = rhs.q - lhs.q;
        match (dp, dq) {
            (1, 1) => Some(Orientation::H(true)),
            (1, 0) => Some(Orientation::P(true)),
            (0, 1) => Some(Orientation::Q(true)),
            (-1, -1) => Some(Orientation::H(false)),
            (-1, 0) => Some(Orientation::P(false)),
            (0, -1) => Some(Orientation::Q(false)),
            _ => None,
        }
    }
    pub fn goto_points(&self, lhs: &CubePoint) -> CubePoint {
        match self {
            Orientation::H(true) => CubePoint::new(lhs.p + 1, lhs.q + 1),
            Orientation::P(true) => CubePoint::new(lhs.p + 1, lhs.q),
            Orientation::Q(true) => CubePoint::new(lhs.p, lhs.q + 1),
            Orientation::H(false) => CubePoint::new(lhs.p - 1, lhs.q - 1),
            Orientation::P(false) => CubePoint::new(lhs.p - 1, lhs.q),
            Orientation::Q(false) => CubePoint::new(lhs.p, lhs.q - 1),
        }
    }
}
