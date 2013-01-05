use ndarray::Array2;
use rand::rngs::SmallRng;
use rand::SeedableRng;

mod config;
mod build;

pub struct Maze2DConfig {
    pub width: usize,
    pub height: usize,
    pub entry: (usize, usize),
    pub exit: (usize, usize),
    pub seed: [u8; 32],
}

pub struct Maze2D {
    config: Maze2DConfig,
    grid: Array2<bool>,
}

