use image::{Rgba, RgbaImage};
use maze_core::{
    square,
    square::{Direction, Joint, Maze2D},
};

pub struct MazeLineRenderer {
    block_size: usize,
    /// notice it's the half width of the wall
    ///
    /// because the wall will draw twice
    wall_width: usize,
    wall_color: Rgba<u8>,
}

impl Default for MazeLineRenderer {
    fn default() -> Self {
        Self { block_size: 10, wall_width: 2, wall_color: Rgba([0, 0, 0, 255]) }
    }
}

impl MazeLineRenderer {
    pub fn new(size: usize) -> Self {
        Self { block_size: size, ..Default::default() }
    }
    pub fn get_walls(&self, maze: &Maze2D) -> Vec<Joint> {
        let mut joints = Vec::new();
        let m01 = maze.matrix01();
        let (width, height) = m01.dim();
        for x in 0..width {
            for y in 0..height {
                if m01[[x, y]] {
                    continue;
                }
                if x > 0 && m01[[x - 1, y]] {
                    joints.push(Joint::new(x, y, Direction::Left));
                }
                if x < width - 1 && m01[[x + 1, y]] {
                    joints.push(Joint::new(x, y, Direction::Right));
                }
                if y > 0 && m01[[x, y - 1]] {
                    joints.push(Joint::new(x, y, Direction::Up));
                }
                if y < height - 1 && m01[[x, y + 1]] {
                    joints.push(Joint::new(x, y, Direction::Down));
                }
            }
        }
        joints
    }
    pub fn render_image_2d(&self, maze: &Maze2D) -> RgbaImage {
        let m01 = maze.matrix01();
        let width = self.block_size * m01.ncols();
        let height = self.block_size * m01.nrows();
        let mut image = RgbaImage::new(width as u32, height as u32);
        for wall in self.get_walls(maze) {
            self.render_wall(&mut image, &wall);
        }
        image
    }
    fn render_wall(&self, image: &mut RgbaImage, joint: &Joint) {
        let x = joint.x * self.block_size;
        let y = joint.y * self.block_size;
        match joint.direction {
            Direction::Up => {
                for i in 0..self.wall_width {
                    for j in 0..self.block_size {
                        let x = x + j;
                        let y = y - i;
                        image.put_pixel(x as u32, y as u32, self.wall_color);
                    }
                }
            }
            Direction::Down => {
                for i in 0..self.wall_width {
                    for j in 0..self.block_size {
                        let x = x + j;
                        let y = y + self.block_size + i;
                        image.put_pixel(x as u32, y as u32, self.wall_color);
                    }
                }
            }
            Direction::Left => {
                for i in 0..self.wall_width {
                    for j in 0..self.block_size {
                        let x = x - i;
                        let y = y + j;
                        image.put_pixel(x as u32, y as u32, self.wall_color);
                    }
                }
            }
            Direction::Right => {
                for i in 0..self.wall_width {
                    for j in 0..self.block_size {
                        let x = x + self.block_size + i;
                        let y = y + j;
                        image.put_pixel(x as u32, y as u32, self.wall_color);
                    }
                }
            }
        }
    }
}
