use super::*;
use std::ops::{Neg, Not};

impl Iterator for Orientation {
    type Item = Orientation;

    // Rotate 60° counterclockwise
    fn next(&mut self) -> Option<Self::Item> {
        *self = match self {
            Orientation::S(true) => Orientation::R(true),
            Orientation::S(false) => Orientation::R(false),
            Orientation::R(true) => Orientation::Q(true),
            Orientation::R(false) => Orientation::Q(false),
            Orientation::Q(true) => Orientation::S(true),
            Orientation::Q(false) => Orientation::S(false),
        };
        Some(*self)
    }
}

impl Not for Orientation {
    type Output = Self;

    fn not(self) -> Self::Output {
        !self
    }
}

impl Neg for Orientation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Orientation::S(v) => Orientation::S(!v),
            Orientation::R(v) => Orientation::R(!v),
            Orientation::Q(v) => Orientation::Q(!v),
        }
    }
}
