use crate::{point::AxialPoint, Direction, SPoint};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Joint {
    point: AxialPoint,
    direction: Direction,
}

impl Joint {
    pub fn new(point: AxialPoint, direction: Direction) -> Self {
        Self { point, direction }
    }
    pub fn from_points(source: AxialPoint, target: AxialPoint) -> Self {
        let SPoint { q: sq, s: ss, r: sr } = source.into();
        let SPoint { q: tq, s: ts, r: tr } = target.into();
        if sq == tq {
            if ss == ts + 1 {
                return Joint::new(source, Direction::S(true));
            }
            if ss == ts - 1 {
                return Joint::new(target, Direction::S(false));
            }
        }
        else if ss == ts {
            if sq == tq + 1 {
                return Joint::new(source, Direction::Q(true));
            }
            if sq == tq - 1 {
                return Joint::new(target, Direction::Q(false));
            }
        }
        else if sr == tr {
            if sq == tq + 1 {
                return Joint::new(source, Direction::R(true));
            }
            if sq == tq - 1 {
                return Joint::new(target, Direction::R(false));
            }
        }
        panic!("{source:?} and {target:?} are not adjacent points")
    }
    pub fn source(&self) -> AxialPoint {
        self.point
    }
    pub fn target(&self) -> AxialPoint {
        self.point.go(self.direction)
    }
    pub fn get_direction(&self) -> Direction {
        self.direction
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
