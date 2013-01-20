use ndarray::Array2;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::{iter::from_generator, ops::Range};
use taxicab_map::Direction;

mod build_dfs;
mod build_prim;
mod display;
mod joint;
mod maze;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Maze2DConfig {
    width: usize,
    height: usize,
    entry: Option<(usize, usize)>,
    exit: Option<(usize, usize)>,
    room: Option<RoomConfig>,
    bad: Vec<(usize, usize)>,
    seed: [u8; 32],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomConfig {
    pub width: Range<usize>,
    pub height: Range<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Room {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Joint {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Maze2D {
    config: Maze2DConfig,
    joints: Vec<Joint>,
    rooms: Vec<Room>,
    solution: Vec<Joint>,
}

impl Default for Maze2DConfig {
    fn default() -> Self {
        let seed = rand::random::<[u8; 32]>();
        Self { width: 5, height: 5, entry: None, exit: None, room: None, bad: vec![], seed }
    }
}

impl Maze2DConfig {
    /// Create a new Maze2DConfig with give size
    ///
    /// # Arguments
    ///
    /// * `width`: width of the maze
    /// * `height`: height of the maze
    ///
    /// # Examples
    ///
    /// ```
    /// use deus_maze::Maze2DConfig;
    /// ```
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
    pub fn new_seed(&mut self) -> [u8; 32] {
        let seed = rand::random::<[u8; 32]>();
        self.set_seed(seed);
        seed
    }
    pub fn with_seed(mut self, seed: [u8; 32]) -> Self {
        self.set_seed(seed);
        self
    }
    pub fn get_rng(&self) -> SmallRng {
        SmallRng::from_seed(self.seed)
    }
}
