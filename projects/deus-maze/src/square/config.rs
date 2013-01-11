use super::*;


impl Default for Maze2DConfig {
    fn default() -> Self {
        let seed = rand::random::<[u8; 32]>();
        Self {
            width: 10,
            height: 10,
            entry: (1, 0),
            exit: (9, 8),
            room: None,
            seed,
        }
    }
}


impl Maze2DConfig {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }

    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.width = width;
        self.height = height;
        self
    }


    pub fn rng(&self) -> SmallRng {
        SmallRng::from_seed(self.seed)
    }
}
