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
    pub fn render_image_2d(&self, maze: &Maze2D) -> RgbaImage {
        let (w, h) = maze.get_size();
        let w = self.block_size * w;
        let h = self.block_size * h;
        let mut image = RgbaImage::new(w as u32, h as u32);
        for wall in maze.get_walls() {
            self.render_wall(&mut image, &wall);
        }
        image
    }
    fn render_wall(&self, image: &mut RgbaImage, joint: &Joint) {
        match joint.direction {
            Direction::Up => {
                let start_x = joint.x * self.block_size;
                let start_y = joint.y * self.block_size;
                self.render_rect(image, start_x, start_y, self.block_size, self.wall_width);
            }
            Direction::Down => {
                let start_x = joint.x * self.block_size;
                let start_y = (joint.y + 1) * self.block_size - self.wall_width;
                self.render_rect(image, start_x, start_y, self.block_size, self.wall_width);
            }
            Direction::Left => {
                let start_x = joint.x * self.block_size;
                let start_y = joint.y * self.block_size;
                self.render_rect(image, start_x, start_y, self.wall_width, self.block_size);
            }
            Direction::Right => {
                let start_x = (joint.x + 1) * self.block_size - self.wall_width;
                let start_y = joint.y * self.block_size;
                self.render_rect(image, start_x, start_y, self.wall_width, self.block_size);
            }
        }
    }
    fn render_rect(&self, image: &mut RgbaImage, x: usize, y: usize, width: usize, height: usize) {
        for i in x..x + width {
            for j in y..y + height {
                image.put_pixel(i as u32, j as u32, self.wall_color);
            }
        }
    }
}
