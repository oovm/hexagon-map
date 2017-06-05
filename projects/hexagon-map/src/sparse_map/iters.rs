use super::*;
use std::collections::{
    btree_map::{Iter, IterMut, Keys},
    BTreeSet,
};

pub struct GetHexagonPoints<'i, T> {
    map: Iter<'i, AxialPoint, T>,
}

pub struct MutGetHexagonPoints<'i, T> {
    map: IterMut<'i, AxialPoint, T>,
}

impl<'i, T> Iterator for GetHexagonPoints<'i, T> {
    type Item = (AxialPoint, &'i T);

    fn next(&mut self) -> Option<Self::Item> {
        let (p, v) = self.map.next()?;
        Some((*p, v))
    }
}

impl<'i, T> Iterator for MutGetHexagonPoints<'i, T> {
    type Item = (AxialPoint, &'i T);

    fn next(&mut self) -> Option<Self::Item> {
        let (p, v) = self.map.next()?;
        Some((*p, v))
    }
}

impl<T> HexagonMap<T> {
    pub fn points_all(&self) -> GetHexagonPoints<T> {
        GetHexagonPoints { map: self.sparse.iter() }
    }
    pub fn points_mut(&mut self) -> MutGetHexagonPoints<T> {
        MutGetHexagonPoints { map: self.sparse.iter_mut() }
    }
}

pub struct HexagonPointsAround<'i, T> {
    keys: Keys<'i, AxialPoint, T>,
    center: AxialPoint,
    distance: isize,
    current: isize,
}

impl<'i, T> Iterator for HexagonPointsAround<'i, T> {
    type Item = AxialPoint;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T> HexagonMap<T> {
    /// Count all defined points in the map.
    pub fn points_count(&self) -> usize {
        self.sparse.len()
    }
    /// Find at most 6 points that are exists and adjacent to given point.
    pub fn points_nearby(&self, source: AxialPoint) -> Vec<AxialPoint> {
        let mut out = Vec::with_capacity(6);
        for direction in Orientation::all() {
            let target = source.go(direction);
            if self.sparse.contains_key(&target) {
                out.push(target);
            }
        }
        out
    }
    /// Find at most 6 joints that are exists and adjacent to given point.
    pub fn joints_nearby(&self, source: AxialPoint) -> Vec<Joint> {
        let mut out = Vec::with_capacity(6);
        for direction in Orientation::all() {
            let target = source.go(direction);
            if self.sparse.contains_key(&target) {
                out.push(source.as_joint(direction));
            }
        }
        out
    }
    /// Find all points that are within a certain distance of a point.
    pub fn points_around(&self, source: AxialPoint, steps: usize) -> HexagonPointsAround<T> {
        HexagonPointsAround { keys: self.sparse.keys(), center: source, distance: steps as isize, current: 0 }
    }
}
