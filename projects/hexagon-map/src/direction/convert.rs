use super::*;
use std::ops::{Neg, Not};

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
