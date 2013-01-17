pub struct MazeHexConfig {
    width: usize,
    height: usize,
    entry: Option<(usize, usize)>,
    exit: Option<(usize, usize)>,
    seed: [u8; 32],
}

impl Default for MazeHexConfig {
    fn default() -> Self {
        Self { width: 10, height: 10, entry: None, exit: None, seed: rand::random::<[u8; 32]>() }
    }
}

impl MazeHexConfig {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, ..Default::default() }
    }
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }
    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.set_size(width, height);
        self
    }
    pub fn get_entry(&self) -> (usize, usize) {
        self.entry.unwrap_or((0, 0))
    }
    pub fn set_entry(&mut self, x: usize, y: usize) {
        self.entry = Some((x, y));
    }
    pub fn with_entry(mut self, x: usize, y: usize) -> Self {
        self.set_entry(x, y);
        self
    }
    pub fn get_exit(&self) -> (usize, usize) {
        self.exit.unwrap_or((self.width - 1, self.height - 1))
    }
    pub fn set_exit(&mut self, x: usize, y: usize) {
        self.exit = Some((x, y));
    }
    pub fn with_exit(mut self, x: usize, y: usize) -> Self {
        self.set_exit(x, y);
        self
    }
    pub fn get_seed(&self) -> [u8; 32] {
        self.seed
    }
    pub fn set_seed(&mut self, seed: [u8; 32]) {
        self.seed = seed;
    }
    pub fn with_seed(mut self, seed: [u8; 32]) -> Self {
        self.set_seed(seed);
        self
    }
}
