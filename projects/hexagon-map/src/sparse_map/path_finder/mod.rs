use super::*;
use std::collections::BTreeSet;

pub struct PathFinder<'a, T> {
    map: &'a HexagonMap<T>,
    start: AxialPoint,
    end: AxialPoint,
    open: BTreeSet<AxialPoint>,
    close: BTreeSet<AxialPoint>,
}

impl<T> HexagonMap<T> {
    pub fn path_finder(&self, start: &AxialPoint, end: &AxialPoint) -> PathFinder<T> {
        PathFinder { map: self, start: *start, end: *end, open: Default::default(), close: Default::default() }
    }
}

impl<'a, T> PathFinder<'a, T> {
    pub fn find(&mut self) -> Option<Vec<AxialPoint>> {
        let mut open = BTreeSet::new();
        let mut close = BTreeSet::new();
        let mut came_from = BTreeMap::new();
        let mut g_score = BTreeMap::new();
        let mut f_score = BTreeMap::new();
        open.insert(self.start);
        g_score.insert(self.start, 0);
        f_score.insert(self.start, self.start.distance(&self.end));
        while !open.is_empty() {
            let current = open.iter().min_by_key(|p| f_score.get(p).unwrap()).unwrap().clone();
            if current == self.end {
                return Some(reconstruct_path(&came_from, &current));
            }
            open.remove(&current);
            close.insert(current);
            for neighbor in self.map.neighbors(&current) {
                if close.contains(&neighbor) {
                    continue;
                }
                let tentative_g_score = g_score.get(&current).unwrap() + current.distance(&neighbor);
                if !open.contains(&neighbor) {
                    open.insert(neighbor);
                }
                else if tentative_g_score >= *g_score.get(&neighbor).unwrap() {
                    continue;
                }
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + neighbor.distance(&self.end));
            }
        }
        None
    }
}
