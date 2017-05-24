use std::collections::{BTreeMap};
use std::collections::btree_map::Iter;
use crate::point::AxialPoint;

/// A sparse hexagon map, if your map size will grow, or most areas will be blank, this is a better choice.
pub struct HexagonMap<T> {
    map: BTreeMap<AxialPoint, T>,
}

impl<T: Default> HexagonMap<T> {
    pub fn circle(diameter: usize) -> Self {
        let mut map = BTreeMap::new();
        for x in 0..diameter {
            for y in 0..diameter {
                let point = AxialPoint::new(x as isize, y as isize);
                map.insert(point, Default::default());
            }
        }
        Self { map }
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

impl<T> HexagonMap<T> {
    /// Get the value at a point, if it exists.
    pub fn get_point(&self, point: AxialPoint) -> Option<&T> {
        self.map.get(&point)
    }
    /// Add a point to the map, if it already exists, return the old value.
    pub fn add_point(&mut self, point: AxialPoint, value: T) -> Option<T> {
        self.map.insert(point, value)
    }
    /// Get a mutable reference to a point, if it exists.
    pub fn mut_point(&mut self, point: AxialPoint) -> Option<&mut T> {
        self.map.get_mut(&point)
    }
    /// Remove a point from the map, if it exists, return the old value.
    pub fn remove_point(&mut self, point: AxialPoint) -> Option<T> {
        self.map.remove(&point)
    }
    pub fn points(&self) -> impl Iterator<Item=&AxialPoint> {
        self.map.keys()
    }
}

impl<'i, T> IntoIterator for &'i HexagonMap<T> {
    type Item = (&'i AxialPoint, &'i T);
    type IntoIter = Iter<'i, AxialPoint, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}
