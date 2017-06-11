use super::*;

use crate::HexPoint;
use ordered_float::OrderedFloat;
use pathfinding::prelude::astar;

/// A* path finder on a hexagon map.
pub struct PathFinder<'a, T> {
    map: &'a HexagonMap<T>,
    start: CubicPoint,
    end: CubicPoint,
    passable: Box<dyn Fn(&CubicPoint, &T) -> bool>,
    action_cost: Box<dyn Fn(&CubicPoint, &T) -> f64>,
}

impl<T> HexagonMap<T> {
    /// Set the passable function.
    ///
    /// # Arguments
    ///
    /// * `passable`:  A function that returns true if the point is passable.
    ///
    /// returns: PathFinder<T>
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexagon_map::HexagonMap;
    /// ```
    pub fn path_finder(&self, start: CubicPoint, end: CubicPoint) -> PathFinder<T> {
        PathFinder { map: self, start, end, passable: Box::new(|_, _| true), action_cost: Box::new(|_, _| 1.0) }
    }
}

impl<'a, T> PathFinder<'a, T> {
    /// Set the passable function.
    ///
    /// # Arguments
    ///
    /// * `passable`:  A function that returns true if the point is passable.
    ///
    /// returns: PathFinder<T>
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexagon_map::HexagonMap;
    /// ```
    pub fn with_passable<F>(mut self, passable: F) -> Self
    where
        F: Fn(&CubicPoint, &T) -> bool + 'static,
    {
        self.passable = Box::new(passable);
        self
    }
    /// Set the passable function.
    ///
    /// # Arguments
    ///
    /// * `passable`:  A function that returns true if the point is passable.
    ///
    /// returns: PathFinder<T>
    ///
    /// # Examples
    ///
    /// ```
    /// # use hexagon_map::HexagonMap;
    /// ```
    pub fn with_cost<F>(mut self, cost: F) -> Self
    where
        F: Fn(&CubicPoint, &T) -> f64 + 'static,
    {
        self.action_cost = Box::new(cost);
        self
    }
    pub fn get_point(&self, point: &CubicPoint) -> Option<&T> {
        self.map.sparse.get(point)
    }
    pub fn has_point(&self, point: &CubicPoint) -> bool {
        self.map.sparse.contains_key(point)
    }
    fn distance(&self, point: &CubicPoint) -> OrderedFloat<f64> {
        OrderedFloat(self.end.taxicab_distance(*point) as f64)
    }
    /// Get all passable neighbors from a point
    pub fn neighbors(&self, point: &CubicPoint) -> Vec<(CubicPoint, OrderedFloat<f64>)> {
        let mut neighbors = vec![];
        for direction in Orientation::all() {
            if let Some(target) = self.map.sparse.get(&point.go(direction)) {
                if (self.passable)(point, target) {
                    let cost = (self.action_cost)(point, target);
                    neighbors.push((point.go(direction), OrderedFloat(cost)));
                }
            }
        }
        neighbors
    }
    pub fn solve_points(self) -> (Vec<CubicPoint>, f64) {
        astar(&self.start, |point| self.neighbors(point), |point| self.distance(point), |point| self.end.eq(point))
            .map(|(path, cost)| (path, cost.0))
            .unwrap_or((vec![], f64::INFINITY))
    }
    pub fn solve_joint(self) -> (Vec<Joint>, f64) {
        todo!()
    }
}
