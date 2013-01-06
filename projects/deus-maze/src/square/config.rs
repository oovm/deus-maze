use super::*;


impl Default for Maze2DConfig {
    fn default() -> Self {
        let seed = rand::random::<[u8; 32]>();
        Self {
            width: 10,
            height: 10,
            entry: (1, 0),
            exit: (9, 8),
            seed,
        }
    }
}

impl Default for Dungeon2DConfig {
    fn default() -> Self {
        let seed = rand::random::<[u8; 32]>();
        Self {
            path: Default::default(),
            room_width: 3..5,
            room_height: 3..5,
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


    pub fn rng(&self) -> rand::rngs::StdRng {
        SmallRng::from_seed(self.seed).into()
    }
}

impl Maze2DConfig {
    pub fn as_dungeon(&self) -> Dungeon2DConfig {
        Dungeon2DConfig {
            width: self.width,
            height: self.height,
            entry: self.entry,
            exit: self.exit,
            room_width: 3..5,
            room_height: 3..5,
            seed: self.seed,
        }
    }
}