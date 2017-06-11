use crate::{CubicPoint, HexPoint};
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
    pub fn from_points<L, R>(lhs: L, rhs: R) -> Option<Self>
    where
        L: HexPoint,
        R: HexPoint,
    {
        let lhs = lhs.as_cubic_point();
        let rhs = rhs.as_cubic_point();
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
    pub fn goto_points(&self, lhs: CubicPoint) -> CubicPoint {
        let CubicPoint { p, q } = lhs;
        let cubic = match self {
            Orientation::H(true) => CubicPoint::new(p + 1, q + 1),
            Orientation::P(true) => CubicPoint::new(p + 1, q),
            Orientation::Q(true) => CubicPoint::new(p, q + 1),
            Orientation::H(false) => CubicPoint::new(p - 1, q - 1),
            Orientation::P(false) => CubicPoint::new(p - 1, q),
            Orientation::Q(false) => CubicPoint::new(p, q - 1),
        };
        cubic
    }
}
