use super::*;

use std::collections::btree_map::{Iter, IterMut};

pub struct GetHexagonPoints<'i, T> {
    map: Iter<'i, CubePoint, T>,
}

pub struct MutGetHexagonPoints<'i, T> {
    map: IterMut<'i, CubePoint, T>,
}

impl<'i, T> Iterator for GetHexagonPoints<'i, T> {
    type Item = (CubePoint, &'i T);

    fn next(&mut self) -> Option<Self::Item> {
        let (p, v) = self.map.next()?;
        Some((*p, v))
    }
}

impl<'i, T> Iterator for MutGetHexagonPoints<'i, T> {
    type Item = (CubePoint, &'i T);

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

pub struct GetHexagonAround<'i, T> {
    map: &'i HexagonMap<T>,
    current: IsometricLine,
}

pub struct MutHexagonAround<'i, T> {
    map: &'i mut HexagonMap<T>,
    current: IsometricLine,
}

impl<'i, T> Iterator for GetHexagonAround<'i, T> {
    type Item = (CubePoint, &'i T);

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.current.next()?;
        match self.map.sparse.get(&p) {
            Some(s) => Some((p, s)),
            None => self.next(),
        }
    }
}

impl<'i, T> Iterator for MutHexagonAround<'i, T> {
    type Item = (CubePoint, &'i mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.current.next()?;
        match self.map.sparse.get_mut(&p) {
            Some(s) => Some((p, s)),
            None => self.next(),
        };
        todo!()
    }
}

impl<T> HexagonMap<T> {
    /// Count all defined points in the map.
    pub fn points_count(&self) -> usize {
        self.sparse.len()
    }
    /// Find at most 6 points that are exists and adjacent to given point.
    pub fn points_nearby(&self, source: CubePoint) -> Vec<CubePoint> {
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
    pub fn joints_nearby(&self, source: CubePoint) -> Vec<Joint> {
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
    pub fn points_around(&self, source: CubePoint, steps: usize) -> GetHexagonAround<T> {
        GetHexagonAround { map: self, current: IsometricLine::new(source, steps) }
    }
}
