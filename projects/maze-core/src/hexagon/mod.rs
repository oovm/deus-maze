pub struct MazeHexConfig {
    size: [usize; 3],
    entry: Option<[usize; 3]>,
    exit: Option<[usize; 3]>,
    seed: [u8; 32],
}

impl Default for MazeHexConfig {
    fn default() -> Self {
        Self { size: [5, 5, 5], entry: None, exit: None, seed: rand::random::<[u8; 32]>() }
    }
}

pub struct Joint {
    x: usize,
    y: usize,
    z: usize,
    direction: Direction,
}

pub enum Direction {
    XU,
    XD,
    YU,
    YD,
    ZU,
    ZD,
}

impl MazeHexConfig {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { size: [x, y, z], ..Default::default() }
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
