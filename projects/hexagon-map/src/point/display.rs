use super::*;
use crate::CubePoint;

impl Debug for AxialPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Axial").field("h", &(-self.p - self.q)).field("p", &self.p).field("q", &self.q).finish()
    }
}

impl Debug for CubePoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cube").field("h", &self.h).field("p", &self.p).field("q", &self.q).finish()
    }
}

impl Display for AxialPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Axial").field(&(-self.p - self.q)).field(&self.p).field(&self.q).finish()
    }
}

impl Display for CubePoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Cube").field(&self.h).field(&self.p).field(&self.q).finish()
    }
}
