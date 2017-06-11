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
    fn as_joint(&self, direction: Orientation) -> Joint {
        Joint::new(*self, direction)
    }
    fn go(&self, direction: Orientation) -> CubicPoint {
        self.as_joint(direction).target()
    }
    /// Get the pixel coordinates of the center of the hexagon
    fn get_pixel_center(&self, radius: f64) -> (f64, f64) {
        let axial = self.as_axial_point();
        let x = radius * 3.0f64.sqrt() * (axial.q as f64 + axial.r as f64 / 2.0);
        let y = radius * 3.0 / 2.0 * axial.r as f64;
        (x, y)
    }
    /// Get the pixel coordinates of the corners of the hexagon
    fn get_pixel_corners(&self, radius: f64) -> [(f64, f64); 6] {
        let (center_x, center_y) = self.get_pixel_center(radius);
        let mut corners = [(0.0, 0.0); 6];
        for i in 0..6 {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / 6.0;
            corners[i] = (center_x + radius * angle.cos(), center_y + radius * angle.sin());
        }
        corners
    }
    /// Calculate the manhattan distance between two points
    fn taxicab_distance<P>(&self, other: P) -> usize
    where
        P: HexPoint,
    {
        let lhs = self.as_axial_point();
        let rhs = other.as_axial_point();
        let add = (lhs.q - rhs.q).abs() + (lhs.r - rhs.r).abs();
        add as usize
    }
    /// Calculate the euclidean distance between two points
    fn euclidean_distance<P>(&self, other: P, radius: f64) -> f64
    where
        P: HexPoint,
    {
        let (x1, y1) = self.get_pixel_center(radius);
        let (x2, y2) = other.get_pixel_center(radius);
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }
}
