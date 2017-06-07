use crate::{direction::Orientation, CubePoint, Joint};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub mod h_point;
pub mod s_point;
pub mod w_point;

mod convert;
mod display;

/// A point in axial coordinates, standard form of a hexagon map
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AxialPoint {
    pub p: isize,
    pub q: isize,
}

impl AxialPoint {
    /// Create a new point in axial coordinates
    pub fn new(p: isize, q: isize) -> Self {
        Self { p, q }
    }
    /// Create a new point in axial coordinates from pixel coordinates
    pub fn from_pixel(x: f64, y: f64, radius: f64) -> Self {
        let q = (x * 3.0f64.sqrt() / 3.0 - y / 3.0) / radius;
        let r = y * 2.0 / 3.0 / radius;
        Self::new(q.round() as isize, r.round() as isize)
    }
    /// Get the pixel coordinates of the center of the hexagon
    pub fn get_center(&self, radius: f64) -> (f64, f64) {
        let x = radius * 3.0f64.sqrt() * (self.p as f64 + self.q as f64 / 2.0);
        let y = radius * 3.0 / 2.0 * self.q as f64;
        (x, y)
    }
    /// Get the pixel coordinates of the corners of the hexagon
    pub fn get_corners(&self, radius: f64) -> [(f64, f64); 6] {
        let (center_x, center_y) = self.get_center(radius);
        let mut corners = [(0.0, 0.0); 6];
        for i in 0..6 {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / 6.0;
            corners[i] = (center_x + radius * angle.cos(), center_y + radius * angle.sin());
        }
        corners
    }
}

impl AxialPoint {
    /// Get the pixel coordinates of the center of the hexagon
    pub fn as_joint(&self, direction: Orientation) -> Joint {
        Joint::new(*self, direction)
    }
    pub fn as_cube_point(&self) -> CubePoint {
        CubePoint::from(*self)
    }
    pub fn go(&self, direction: Orientation) -> Self {
        self.as_joint(direction).target()
    }
    /// Calculate the euclidean distance between two points
    pub fn euclidean_distance(&self, other: &Self, radius: f64) -> f64 {
        let lhs = self.get_center(radius);
        let rhs = other.get_center(radius);
        ((lhs.0 - rhs.0).powi(2) + (lhs.1 - rhs.1).powi(2)).sqrt()
    }
    /// Calculate the manhattan distance between two points
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.p - other.p).abs() + (self.q - other.q).abs()) as usize
    }
}
