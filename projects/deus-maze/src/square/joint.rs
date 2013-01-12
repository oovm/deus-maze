use super::*;

impl Joint {
    pub fn all(x: usize, y: usize, width: usize, height: usize) -> Vec<Self> {
        let mut joints = Vec::with_capacity(4);
        if y > 0 {
            joints.push(Self { x, y, direction: Direction::Up })
        }
        if y < height - 1 {
            joints.push(Self { x, y, direction: Direction::Down })
        }
        if x > 0 {
            joints.push(Self { x, y, direction: Direction::Left })
        }
        if x < width - 1 {
            joints.push(Self { x, y, direction: Direction::Right })
        }
        joints
    }
    pub fn source(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    pub fn target(&self) -> (usize, usize) {
        match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        }
    }
    pub fn render_path(width: usize, height: usize, joints: &[Joint]) -> Array2<bool> {
        let size = (width * 2 + 1, height * 2 + 1);
        let mut matrix = Array2::from_elem(size, false);
        for joint in joints {
            let (x, y) = joint.source();
            let (x, y) = (x * 2 + 1, y * 2 + 1);
            matrix[[x, y]] = true;
            match joint.direction {
                Direction::Up => matrix[[x, y - 1]] = true,
                Direction::Down => matrix[[x, y + 1]] = true,
                Direction::Left => matrix[[x - 1, y]] = true,
                Direction::Right => matrix[[x + 1, y]] = true,
            }
        }
        matrix
    }
}

impl Maze2D {
    pub fn render(&self) -> Array2<bool> {
        Joint::render_path(self.config.width, self.config.height, &self.joints)
    }
}
