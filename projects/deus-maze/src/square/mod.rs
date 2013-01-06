use std::ops::Range;
use ndarray::Array2;
use rand::rngs::SmallRng;
use rand::SeedableRng;

mod config;
mod build_path;
mod build_dungeon;

pub struct Maze2DConfig {
    pub width: usize,
    pub height: usize,
    pub entry: (usize, usize),
    pub exit: (usize, usize),
    pub seed: [u8; 32],
}

pub struct Dungeon2DConfig {
    pub width: usize,
    pub height: usize,
    pub entry: (usize, usize),
    pub exit: (usize, usize),
    pub room_width: Range<usize>,
    pub room_height: Range<usize>,
    pub seed: [u8; 32],
}


pub struct Maze2D {
    config: Maze2DConfig,
    grid: Array2<bool>,
}

