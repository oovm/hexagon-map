use super::*;
use std::collections::btree_map::{Iter, IterMut};

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

impl<T> HexagonMap<T> {
    /// Count all defined points in the map.
    pub fn point_count(&self) -> usize {
        self.sparse.len()
    }
    /// Find at most 6 points that are exists and adjacent to a point.
    pub fn points_nearby(&self, from: &AxialPoint) -> Vec<AxialPoint> {
        from.nearby().into_iter().filter(|p| self.sparse.contains_key(p)).collect()
    }
    pub fn joints_nearby(&self, from: &AxialPoint) -> Vec<AxialPoint> {
        from.nearby().into_iter().filter(|p| self.sparse.contains_key(p) && self.sparse.contains_key(&p.joint())).collect()
    }
    /// Find all points that are within a certain distance of a point.
    pub fn points_around(&self, from: &AxialPoint, distance: usize) -> Vec<AxialPoint> {
        match distance {
            0 => vec![*from],
            1 => self.points_nearby(from),
            // TODO: optimize this
            _ => {
                let mut points = vec![];
                for point in self.points_nearby(from) {
                    points.extend(self.points_around(&point, distance - 1));
                }
                points
            }
        }
    }
}
