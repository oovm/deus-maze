use image::{Rgba, RgbaImage};
use maze_core::{
    square,
    square::{Direction, Joint, Maze2D},
};

pub struct MazeLineRenderer {
    block_size: usize,
    wall_width_half: usize,
    wall_color: Rgba<u8>,
}

impl Default for MazeLineRenderer {
    fn default() -> Self {
        Self { block_size: 10, wall_width_half: 2, wall_color: Rgba([0, 0, 0, 255]) }
    }
}

impl MazeLineRenderer {
    pub fn new(size: usize) -> Self {
        Self { block_size: size, ..Default::default() }
    }
    pub fn set_wall_width(&mut self, width: usize) {
        if cfg!(debug_assertions) {
            if width % 2 == 1 {
                tracing::warn!("Wall width should be an even number.");
            }
        }
        self.wall_width_half = width / 2;
    }
    pub fn render_image_2d(&self, maze: &Maze2D) -> RgbaImage {
        let (w, h) = maze.get_size();
        let w = self.block_size * w;
        let h = self.block_size * h;
        let mut image = RgbaImage::new(w as u32, h as u32);
        // self.render_border(&mut image);
        for wall in maze.get_walls() {
            self.render_wall(&mut image, &wall);
        }
        image
    }
    fn render_border(&self, image: &mut RgbaImage) {
        let (w, h) = image.dimensions();
        let w = w as usize;
        let h = h as usize;
        self.render_rect(image, 0, 0, w, self.wall_width_half);
        self.render_rect(image, 0, 0, self.wall_width_half, h);
        self.render_rect(image, 0, h - self.wall_width_half, w, self.wall_width_half);
        self.render_rect(image, w - self.wall_width_half, 0, self.wall_width_half, h);
    }
    fn render_wall(&self, image: &mut RgbaImage, joint: &Joint) {
        match joint.direction {
            Direction::Up => self.render_rect(
                image,
                joint.x * self.block_size,
                joint.y * self.block_size,
                self.block_size + self.wall_width_half,
                self.wall_width_half + self.wall_width_half,
            ),
            Direction::Down => self.render_rect(
                image,
                joint.x * self.block_size,
                joint.y * self.block_size + self.block_size - self.wall_width_half,
                self.block_size + self.wall_width_half,
                self.wall_width_half + self.wall_width_half,
            ),
            Direction::Left => self.render_rect(
                image,
                joint.x * self.block_size,
                joint.y * self.block_size,
                self.wall_width_half + self.wall_width_half,
                self.block_size + self.wall_width_half,
            ),
            Direction::Right => self.render_rect(
                image,
                joint.x * self.block_size + self.block_size - self.wall_width_half,
                joint.y * self.block_size,
                self.wall_width_half + self.wall_width_half,
                self.block_size + self.wall_width_half,
            ),
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
