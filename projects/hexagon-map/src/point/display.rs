use super::*;
use crate::AxialPoint;

impl Debug for CubicPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let h = self.p - self.q;
        f.debug_struct("Cube").field("h", &h).field("p", &self.p).field("q", &self.q).finish()
    }
}

impl Debug for AxialPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = -self.q - self.r;
        f.debug_struct("Axial").field("q", &self.q).field("r", &self.r).field("s", &s).finish()
    }
}

impl Display for CubicPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let h = self.p - self.q;
        f.debug_tuple("Cube").field(&h).field(&self.p).field(&self.q).finish()
    }
}

impl Display for AxialPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = -self.q - self.r;
        f.debug_tuple("Axial").field(&self.q).field(&self.r).field(&s).finish()
    }
}
