use super::*;
use std::ops::{Neg, Not};

impl Iterator for Orientation {
    type Item = Orientation;

    // Rotate 60° counterclockwise
    fn next(&mut self) -> Option<Self::Item> {
        *self = match self {
            Orientation::H(true) => Orientation::P(true),
            Orientation::H(false) => Orientation::P(false),
            Orientation::P(true) => Orientation::Q(true),
            Orientation::P(false) => Orientation::Q(false),
            Orientation::Q(true) => Orientation::H(true),
            Orientation::Q(false) => Orientation::H(false),
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
            Orientation::H(v) => Orientation::H(!v),
            Orientation::P(v) => Orientation::P(!v),
            Orientation::Q(v) => Orientation::Q(!v),
        }
    }
}
