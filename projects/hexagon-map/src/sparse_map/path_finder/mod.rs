use super::*;
use std::collections::BTreeSet;

/// A* path finder on a hexagon map.
pub struct PathFinder<'a, T> {
    map: &'a HexagonMap<T>,
    start: AxialPoint,
    end: AxialPoint,
    open: BTreeSet<AxialPoint>,
    close: BTreeSet<AxialPoint>,
    passable: Box<dyn Fn(&AxialPoint) -> bool>,
    action_point: Option<f64>,
    action_cost: Box<dyn Fn(&AxialPoint) -> f64>,
}

impl<T> HexagonMap<T> {
    pub fn path_finder(&self, start: &AxialPoint, end: &AxialPoint) -> PathFinder<T> {
        PathFinder {
            map: self,
            start: start.clone(),
            end: end.clone(),
            open: Default::default(),
            close: Default::default(),
            passable: Box::new(|f| true),
            action_point: None,
            action_cost: Box::new(()),
        }
    }
}

impl<'a, T> PathFinder<'a, T> {
    pub fn with_passable<F>(mut self, passable: F) -> Self
    where
        F: Fn(&AxialPoint) -> bool,
    {
        self.passable = Box::new(passable);
        self
    }
    pub fn with_action(mut self, action: f64) -> Self {
        if action.is_sign_negative() {
            self.action_point = None;
        }
        else {
            self.action_point = Some(action);
        }
        self
    }
    pub fn with_cost<F>(mut self, cost: F) -> Self
    where
        F: Fn(&AxialPoint) -> f64,
    {
        self.action_point = Some(0.0);
        self.action_cost = Box::new(cost);
        self
    }

    pub fn find<F>(mut self, reject: F) -> Option<Vec<AxialPoint>>
    where
        F: Fn(&AxialPoint) -> bool,
    {
        // A* algorithm
        self.open.insert(self.start);
        while !self.open.is_empty() {
            let current = self.open.iter().min_by_key(|&x| self.map.distance(*x, self.end)).unwrap();
            self.open.remove(current);
            self.close.insert(*current);
            if *current == self.end {
                break;
            }
            for neighbor in self.map.neighbors(*current) {
                if reject(&neighbor) {
                    continue;
                }
                if self.close.contains(&neighbor) {
                    continue;
                }
                self.open.insert(neighbor);
            }
        }
        if self.close.contains(&self.end) {
            let mut path = vec![self.end];
            let mut current = self.end;
            while current != self.start {
                for neighbor in self.map.neighbors(current) {
                    if self.close.contains(&neighbor) {
                        current = neighbor;
                        path.push(current);
                        break;
                    }
                }
            }
            path.reverse();
            Some(path)
        }
        else {
            None
        }
    }
}
