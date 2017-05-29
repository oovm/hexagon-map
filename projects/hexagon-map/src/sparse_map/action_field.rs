use super::*;
use itertools::Itertools;

pub struct CostField<'a, T> {
    map: &'a HexagonMap<T>,
    start: AxialPoint,
    open: BTreeMap<AxialPoint, f64>,
    close: BTreeMap<AxialPoint, f64>,
    action_points: f64,
    passable: Box<dyn Fn(&AxialPoint, &T) -> bool>,
    action_cost: Box<dyn Fn(&AxialPoint, &T) -> f64>,
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
    pub fn cost_field(&self, start: &AxialPoint, action: f64) -> CostField<T> {
        let mut open = BTreeMap::new();
        open.insert(*start, 0.0);
        CostField {
            map: self,
            start: start.clone(),
            open,
            close: Default::default(),
            action_points: action,
            passable: Box::new(|_, _| true),
            action_cost: Box::new(|_, _| 0.0),
        }
    }
}

impl<'a, T> CostField<'a, T> {
    /// Get all passable neighbors from a point
    pub fn add_neighbors(&self, point: &AxialPoint) -> Vec<(AxialPoint, f64)> {
        let mut neighbors = Vec::with_capacity(6);
        for direction in Direction::all() {
            let key = point.go(direction);
            if let Some(value) = self.map.sparse.get(&key) {
                if !(self.passable)(&key, value) {
                    continue;
                }
                if self.close.contains_key(&key) {
                    continue;
                }
                let cost = (self.action_cost)(point, value);
                neighbors.push((key, cost));
            }
        }
        neighbors
    }
    pub fn solve(mut self) -> Vec<(f64, AxialPoint)> {
        while let Some((point, cost)) = self.open.pop_first() {
            for (neighbor, neighbor_cost) in self.add_neighbors(&point) {
                let new_cost = cost + neighbor_cost;
                if new_cost > self.action_points {
                    continue;
                }
                else {
                    self.open.insert(neighbor, new_cost);
                }
            }
            self.close.insert(point, cost);
        }
        self.close.iter().map(|(k, v)| (*v, *k)).sorted_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).collect()
    }
}