use super::*;
use ndarray::s;

impl Maze2D {
    pub fn new(config: &Maze2DConfig, joints: &[Joint], rooms: &[Room]) -> Self {
        Self { config: config.clone(), joints: joints.to_vec(), rooms: rooms.to_vec() }
    }
    pub fn get_polygon_points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();
        for room in &self.rooms {
            points.push((room.x, room.y));
            points.push((room.x + room.width, room.y));
            points.push((room.x + room.width, room.y + room.height));
            points.push((room.x, room.y + room.height));
        }
        points
    }

    pub fn get_rectangles(&self) -> Vec<Room> {
        // find all 1-rectangles in 01-matrix
        let mut m01 = self.matrix01();
        let mut rects = Vec::new();
        for x in 0..m01.shape()[0] {
            for y in 0..m01.shape()[1] {
                if m01[[x, y]] {
                    // find a biggest rectangle
                    let mut w = 1;
                    let mut h = 1;
                    while x + w < m01.shape()[0] {
                        if m01.slice(s![x + w, y..y + h]).iter().all(|&x| x) {
                            w += 1;
                        }
                        else {
                            break;
                        }
                    }
                    while y + h < m01.shape()[1] {
                        if m01.slice(s![x..x + w, y + h]).iter().all(|&x| x) {
                            h += 1;
                        }
                        else {
                            break;
                        }
                    }
                    // remove the rectangle from the matrix
                    for i in x..x + w {
                        for j in y..y + h {
                            m01[[i, j]] = false;
                        }
                    }
                    rects.push(Room::new(x, y, w, h));
                }
            }
        }
        rects
    }
}

impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }
    pub fn set_matrix(&self, m: &mut Array2<bool>, on: bool) {
        for i in self.x..self.x + self.width {
            for j in self.y..self.y + self.height {
                m[[i, j]] = on;
            }
        }
    }
}
