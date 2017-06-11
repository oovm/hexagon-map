use crate::{CubicPoint, HexPoint, Orientation};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Joint {
    point: CubicPoint,
    direction: Orientation,
}

impl Joint {
    pub fn new<P>(point: P, direction: Orientation) -> Self
    where
        P: HexPoint,
    {
        Self { point: point.as_cubic_point(), direction }
    }
    pub fn from_points(source: CubicPoint, target: CubicPoint) -> Self {
        match Orientation::from_points(&source, &target) {
            Some(s) => source.as_joint(s),
            None => panic!("{source:?} and {target:?} are not adjacent points"),
        }
    }
}

impl Joint {
    pub fn source(&self) -> impl HexPoint {
        self.point
    }
    pub fn target(&self) -> impl HexPoint {
        self.direction.goto_points(&self.point)
    }
    pub fn get_direction(&self) -> Orientation {
        self.direction
    }
    pub fn set_direction(&mut self, direction: Orientation) {
        self.direction = direction;
    }
    pub fn forward(&self) -> Self {
        Self::new(self.point.go(self.direction), self.direction)
    }
    pub fn rotate(&self, clockwise: bool) -> Self {
        Self::new(self.point, self.direction.rotate(clockwise))
    }
}
