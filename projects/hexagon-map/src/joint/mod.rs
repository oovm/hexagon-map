use crate::{HDirection, CPoint};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct HJoint {
    pub point: CPoint,
    pub direction: HDirection,
}


impl HJoint {
    pub fn source(&self) -> CPoint {
        self.point
    }
    pub fn target(&self) -> CPoint {
        self.point.go(self.direction)
    }
}