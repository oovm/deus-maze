use super::*;

impl Maze2D {
    pub fn new(config: &Maze2DConfig, joints: &[Joint], rooms: &[Room]) -> Self {
        Self { config: config.clone(), joints: joints.to_vec(), rooms: rooms.to_vec(), solution: vec![] }
    }
    pub fn set_entry(mut self, x: usize, y: usize) {
        self.config.set_entry(x, y);
    }
    pub fn set_exit(mut self, x: usize, y: usize) {
        self.config.set_exit(x, y);
    }
    pub fn get_size(&self) -> (usize, usize) {
        self.config.get_size()
    }
    pub fn get_walls(&self) -> Vec<Joint> {
        let m01 = self.matrix01();
        let mut walls = Vec::new();
        for x in 0..self.config.width {
            for y in 0..self.config.height {
                if !m01[[x * 2 + 0, y * 2 + 1]] {
                    walls.push(Joint::new(x, y, Direction::Left));
                }
                if !m01[[x * 2 + 2, y * 2 + 1]] {
                    walls.push(Joint::new(x, y, Direction::Right));
                }
                if !m01[[x * 2 + 1, y * 2 + 0]] {
                    walls.push(Joint::new(x, y, Direction::Up));
                }
                if !m01[[x * 2 + 1, y * 2 + 2]] {
                    walls.push(Joint::new(x, y, Direction::Down));
                }
            }
        }
        walls
    }
    pub fn solve(&mut self) -> bool {
        if !self.solution.is_empty() {
            return true;
        }
        // let (width, height) = self.get_size();
        // let (entry_x, entry_y) = self.config.get_entry();
        // let (exit_x, exit_y) = self.config.get_exit();
        todo!()
    }
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
}
