use crate::{CubePoint, Joint, Orientation};
use std::fmt::{Debug, Formatter};

pub struct IsometricLine {
    wrapper: IsometricLineSealed,
}

impl Debug for IsometricLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IsometricLine { wrapper: IsometricLineSealed::Zero { center, stop } } => {
                let index = match stop {
                    true => 1,
                    false => 0,
                };
                f.debug_struct("IsometricLine").field("current", center).field("distance", &0).field("index", &index).finish()
            }
            IsometricLine { wrapper: IsometricLineSealed::Normal { current, distance, index } } => f
                .debug_struct("IsometricLine")
                .field("current", &current.target())
                .field("distance", distance)
                .field("index", index)
                .finish(),
        }
    }
}

enum IsometricLineSealed {
    Zero { center: CubePoint, stop: bool },
    Normal { current: Joint, distance: isize, index: isize },
}

impl IsometricLine {
    pub fn new<P>(center: P, distance: usize) -> IsometricLine
    where
        P: Into<CubePoint>,
    {
        let center = center.into();
        let h_offset = distance as isize;
        let wrapper = match distance {
            0 => IsometricLineSealed::Zero { center, stop: false },
            _ => IsometricLineSealed::Normal {
                current: Joint::new(CubePoint { p: center.p + h_offset, q: center.q + h_offset - 1 }, Orientation::P(true)),
                distance: h_offset,
                index: 0,
            },
        };
        IsometricLine { wrapper }
    }
    fn increase(&mut self) {
        match &mut self.wrapper {
            IsometricLineSealed::Zero { stop, .. } => {
                *stop = true;
            }
            IsometricLineSealed::Normal { index, .. } => {
                *index += 1;
            }
        }
    }
    pub fn is_finish(&self) -> bool {
        match &self.wrapper {
            IsometricLineSealed::Zero { stop, .. } => *stop,
            IsometricLineSealed::Normal { index, distance, .. } => *index >= 6 * *distance,
        }
    }
}

impl Iterator for IsometricLine {
    type Item = CubePoint;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = None;
        if !self.is_finish() {
            match &mut self.wrapper {
                IsometricLineSealed::Zero { center, .. } => out = Some(*center),
                IsometricLineSealed::Normal { current, distance, index } => {
                    match *index % *distance {
                        0 => {
                            *current = current.forward().rotate(false);
                            out = Some(current.source());
                        }
                        _ => {
                            *current = current.forward();
                            out = Some(current.source());
                        }
                    };
                }
            }
            self.increase();
        }
        out
    }
}
