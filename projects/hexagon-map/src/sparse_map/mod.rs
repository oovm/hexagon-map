use std::collections::{BTreeMap};
use crate::point::AxialPoint;

/// A sparse hexagon map, if your map size will grow, or most areas will be blank, this is a better choice.
pub struct HexagonMap<T> {
    map: BTreeMap<AxialPoint, T>,
}

impl<T: Default> HexagonMap<T> {
    pub fn circle(diameter: usize) -> Self {
        todo!()
    }
    pub fn rhombus(width: usize, height: usize) -> Self {
        let mut map = BTreeMap::new();
        for x in 0..width {
            for y in 0..height {
                map.insert(AxialPoint::new(x as isize, y as isize), Default::default());
            }
        }
        Self { map }
    }
    pub fn width_first(width: usize, height: usize, fill: (bool, bool)) -> Self {
        todo!()
    }
    pub fn height_first(width: usize, height: usize, fill: (bool, bool)) -> Self {
        todo!()
    }
}