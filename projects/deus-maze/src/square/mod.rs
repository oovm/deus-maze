use std::ops::Range;
use ndarray::Array2;
use rand::rngs::SmallRng;
use rand::SeedableRng;

mod config;
mod build_path;
mod build_dungeon;
mod display;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Maze2DConfig {
    pub width: usize,
    pub height: usize,
    pub entry: (usize, usize),
    pub exit: (usize, usize),
    pub room: Option<RoomConfig>,
    pub seed: [u8; 32],
}

pub struct RoomConfig {
    pub width: Range<usize>,
    pub height: Range<usize>,
}

pub struct Maze2D {
    config: Maze2DConfig,
    grid: Array2<bool>,
}

pub struct Room {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}