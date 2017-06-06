use crate::{point::AxialPoint, Orientation};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Joint {
    point: AxialPoint,
    direction: Orientation,
}

impl Joint {
    pub fn new(point: AxialPoint, direction: Orientation) -> Self {
        Self { point, direction }
    }
    pub fn from_points(source: AxialPoint, target: AxialPoint) -> Self {
        match Orientation::from_points(&source, &target) {
            Some(s) => source.as_joint(s),
            None => panic!("{source:?} and {target:?} are not adjacent points"),
        }
    }
    pub fn source(&self) -> AxialPoint {
        self.point
    }
    pub fn target(&self) -> AxialPoint {
        self.point.go(self.direction)
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
