pub struct MazeHexConfig {
    size: [usize; 3],
    entry: Option<[usize; 3]>,
    exit: Option<[usize; 3]>,
    bad: Vec<[usize; 3]>,
    seed: [u8; 32],
}

impl Default for MazeHexConfig {
    fn default() -> Self {
        Self { size: [5, 5, 5], entry: None, exit: None, bad: vec![], seed: rand::random::<[u8; 32]>() }
    }
}





impl MazeHexConfig {
    pub fn circle(x: usize, y: usize, z: usize) -> Self {
        Self { size: [x, y, z], ..Default::default() }
    }
    pub fn square(x: usize, y: usize) -> Self {
        Self { size: [x, y, 1], ..Default::default() }
    }
    pub fn get_size(&self) -> [usize; 3] {
        self.size
    }
    pub fn set_size(&mut self, x: usize, y: usize, z: usize) {
        self.size = [x, y, z];
    }
    pub fn with_size(mut self, x: usize, y: usize, z: usize) -> Self {
        self.set_size(x, y, z);
        self
    }
}
