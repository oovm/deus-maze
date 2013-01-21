use super::*;
use taxicab_map::TaxicabMap;

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
        let w = self.config.width as isize;
        let h = self.config.height as isize;
        for x in 0..w {
            for y in 0..h {
                if !m01[(x * 2 + 1, y * 2 + 0)] {
                    walls.push(Joint::new(x, y, Direction::Y(true)));
                }
                if !m01[(x * 2 + 1, y * 2 + 2)] {
                    walls.push(Joint::new(x, y, Direction::Y(false)));
                }
                if !m01[(x * 2 + 0, y * 2 + 1)] {
                    walls.push(Joint::new(x, y, Direction::X(false)));
                }
                if !m01[(x * 2 + 2, y * 2 + 1)] {
                    walls.push(Joint::new(x, y, Direction::X(true)));
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

impl Maze2D {
    pub fn matrix01(&self) -> TaxicabMap<bool> {
        let mut matrix = TaxicabMap::rectangle(self.config.width * 2 + 1, self.config.height * 2 + 1, &false);
        for joint in &self.joints {
            let (sx, sy) = joint.source();
            let (tx, ty) = joint.target();
            matrix[(sx * 2 + 1, sy * 2 + 1)] = true;
            matrix[(tx * 2 + 1, ty * 2 + 1)] = true;
            match joint.direction {
                Direction::Y(true) => matrix[(sx * 2 + 1, sy * 2 + 0)] = true,
                Direction::Y(false) => matrix[(sx * 2 + 1, sy * 2 + 2)] = true,
                Direction::X(false) => matrix[(sx * 2 + 0, sy * 2 + 1)] = true,
                Direction::X(true) => matrix[(sx * 2 + 2, sy * 2 + 1)] = true,
            }
        }
        matrix
    }
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
}
