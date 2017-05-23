use crate::{Direction};
use serde::{Serialize, Deserialize};
use crate::point::AxialPoint;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Joint {
    pub point: AxialPoint,
    pub direction: Direction,
}

impl Joint {
    pub fn source(&self) -> AxialPoint {
        self.point
    }
    pub fn target(&self) -> AxialPoint {
        self.point.go(self.direction)
    }
}