pub struct Joint {
    /// Q-axis index
    pub q: usize,
    /// S-axis index
    pub s: usize,
    /// R-axis index
    pub r: usize,
    pub direction: Direction,
}



pub enum Direction {
    Q(bool),
    S(bool),
    R(bool),
}

impl Joint {
    pub fn source(&self) -> [usize; 3] {
        [self.q, self.s, self.r]
    }
    pub fn target(&self) -> [usize; 3] {
        match self.direction {
            Direction::Q(true) => [self.q, self.s + 1, self.r - 1],
            Direction::Q(false) => [self.q, self.s - 1, self.r + 1],
            Direction::S(true) => [self.q + 1, self.s, self.r - 1],
            Direction::S(false) => [self.q - 1, self.s, self.r + 1],
            Direction::R(true) => [self.q + 1, self.s - 1, self.r],
            Direction::R(false) => [self.q - 1, self.s + 1, self.r],
        }
    }
}