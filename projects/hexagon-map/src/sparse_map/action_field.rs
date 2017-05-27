use super::*;
use crate::PathFinder;

pub struct ActionField {
    center: AxialPoint,
    map: Vec<f64, AxialPoint>,
}

impl<'a, T> PathFinder<'a, T> {
    pub fn action_field(mut self) -> ActionField {
        while let Some(point) = self.open.pop_first() {}
        todo!()
    }
}
