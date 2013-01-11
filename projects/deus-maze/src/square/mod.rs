use std::ops::Range;
use ndarray::Array2;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use serde::{Serialize, Deserialize};
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomConfig {
    pub width: Range<usize>,
    pub height: Range<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Maze2D {
    config: Maze2DConfig,
    grid: Array2<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Room {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}