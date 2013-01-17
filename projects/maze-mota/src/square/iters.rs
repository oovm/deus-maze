use super::*;

impl Maze2D {
    pub fn new(config: &Maze2DConfig, joints: &[Joint], rooms: &[Room]) -> Self {
        Self { config: config.clone(), joints: joints.to_vec(), rooms: rooms.to_vec() }
    }
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
}
