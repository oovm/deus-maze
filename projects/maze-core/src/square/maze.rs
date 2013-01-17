use super::*;
use std::collections::VecDeque;

impl Maze2D {
    pub fn new(config: &Maze2DConfig, joints: &[Joint], rooms: &[Room]) -> Self {
        Self { config: config.clone(), joints: joints.to_vec(), rooms: rooms.to_vec() }
    }
    pub fn set_entry(mut self, x: usize, y: usize) {
        self.config.set_entry(x, y);
    }
    pub fn set_exit(mut self, x: usize, y: usize) {
        self.config.set_exit(x, y);
    }
    pub fn solve(&self) -> Vec<(usize, usize)> {
        todo!()
    }
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
}
