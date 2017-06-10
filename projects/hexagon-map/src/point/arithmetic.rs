use super::*;

impl<P> PartialEq<P> for CubicPoint
where
    P: HexPoint,
{
    fn eq(&self, other: &P) -> bool {
        let rhs = other.as_cubic_point();
        self.p == rhs.p && self.q == rhs.q
    }
}

impl<P> PartialEq<P> for AxialPoint
where
    P: HexPoint,
{
    fn eq(&self, other: &P) -> bool {
        let rhs = other.as_axial_point();
        self.q == rhs.q && self.r == rhs.r
    }
}
