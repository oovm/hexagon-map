use crate::{direction::Orientation, AxialPoint, CubicPoint, Joint};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub mod a_point;
pub mod c_point;
pub mod h_point;
pub mod w_point;

mod arithmetic;
mod convert;
mod display;

pub trait HexPoint: Copy + Eq + Ord + Display + Debug {
    /// Create a new point in cubic coordinates
    fn as_cubic_point(&self) -> CubicPoint;
    /// Create a new point in axial coordinates from pixel coordinates
    fn as_axial_point(&self) -> AxialPoint;
}
