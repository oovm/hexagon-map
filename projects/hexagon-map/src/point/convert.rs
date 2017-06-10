use super::*;
use crate::HPoint;

impl From<CubicPoint> for AxialPoint {
    // h -> r
    // p -> -s
    // q -> q
    fn from(point: CubicPoint) -> Self {
        AxialPoint::new(point.get_q(), point.get_h())
    }
}

impl HexPoint for CubicPoint {
    fn as_cubic_point(&self) -> CubicPoint {
        *self
    }
    fn as_axial_point(&self) -> AxialPoint {
        AxialPoint::from(*self)
    }
}

impl From<AxialPoint> for CubicPoint {
    // r -> h
    // q -> q
    // s -> -p
    fn from(point: AxialPoint) -> Self {
        CubicPoint::new(-point.get_s(), point.get_q())
    }
}

impl HexPoint for AxialPoint {
    fn as_cubic_point(&self) -> CubicPoint {
        CubicPoint::from(*self)
    }

    fn as_axial_point(&self) -> AxialPoint {
        *self
    }
}

impl From<CubicPoint> for HPoint {
    fn from(point: CubicPoint) -> Self {
        HPoint::new(point.p, point.q)
    }
}

impl Into<CubicPoint> for HPoint {
    fn into(self) -> CubicPoint {
        CubicPoint::new(self.x, self.y)
    }
}
