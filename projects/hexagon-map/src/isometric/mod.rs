pub enum IsometricLine {
    Zero { center: AxialPoint, stop: bool },
    Normal { current: Joint, distance: isize, index: isize },
}

impl IsometricLine {
    pub fn new(center: AxialPoint, distance: usize) -> Self {
        match distance {
            0 => Self::Zero { center, stop: false },
            _ => Self::Normal {
                current: Joint::new(AxialPoint { p: center.p + distance - 1, q: center.q + 1 }, Orientation::H(true)),
                distance: distance as isize,
                index: 0,
            },
        }
    }
}

impl Iterator for IsometricLine {
    type Item = AxialPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = None;
        if self.distance == 0 && self.index == 0 {
            out = Some(self.current.source());
        }
        else if self.index < 6 * self.distance {
            match self.index % self.distance {
                0 => {
                    let new = self.current.rotate(false).forward();
                    out = Some(new.source());
                }
                _ => {
                    let new = self.current.forward();
                    out = Some(new.source());
                }
            }
        }
        self.index += 1;
        out
    }
}
