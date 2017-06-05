use super::*;
use crate::CubePoint;

impl Debug for AxialPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Axial").field("q", &self.q).field("s", &(-self.q - self.r)).field("r", &self.r).finish()
    }
}

impl Debug for CubePoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cube").field("s", &self.s).field("q", &self.q).field("r", &self.r).finish()
    }
}

impl Display for AxialPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Axial").field(&self.q).field(&(-self.q - self.r)).field(&self.r).finish()
    }
}

impl Display for CubePoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Cube").field(&self.s).field(&self.q).field(&self.r).finish()
    }
}
