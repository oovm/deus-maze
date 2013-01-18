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
        let bw = self.block_size * w;
        let bh = self.block_size * h;
        let mut image = RgbaImage::new(bw as u32, bh as u32);
        for wall in maze.get_walls() {
            self.render_wall(&mut image, &wall, w, h);
        }
        image
    }
    fn render_wall(&self, image: &mut RgbaImage, joint: &Joint, lower: usize, right: usize) {
        let border = self.wall_width_half;
        println!("{} {}", joint.y, lower);
        match joint.direction {
            Direction::Up if joint.y == 0 => self.render_rect(image, joint.x * self.block_size, 0, self.block_size, border * 2),
            Direction::Up => self.render_rect(
                image,
                joint.x * self.block_size,
                joint.y * self.block_size,
                self.block_size + self.wall_width_half,
                border,
            ),
            // lowest wall
            Direction::Down if joint.y == lower - 1 => self.render_rect(
                image,
                joint.x * self.block_size,
                (joint.y + 1) * self.block_size - border * 2,
                self.block_size,
                border * 2,
            ),
            Direction::Down => self.render_rect(
                image,
                joint.x * self.block_size,
                (joint.y + 1) * self.block_size - border,
                self.block_size + self.wall_width_half,
                border,
            ),
            Direction::Left if joint.x == 0 => {
                self.render_rect(image, 0, joint.y * self.block_size, border * 2, self.block_size)
            }
            Direction::Left => self.render_rect(
                image,
                joint.x * self.block_size,
                joint.y * self.block_size,
                border,
                self.block_size + self.wall_width_half,
            ),
            Direction::Right if joint.x == right - 1 => self.render_rect(
                image,
                (joint.x + 1) * self.block_size - border * 2,
                joint.y * self.block_size,
                border * 2,
                self.block_size,
            ),
            Direction::Right => self.render_rect(
                image,
                (joint.x + 1) * self.block_size - border,
                joint.y * self.block_size,
                border,
                self.block_size + self.wall_width_half,
            ),
        }
    }
    fn render_rect(&self, image: &mut RgbaImage, x: usize, y: usize, width: usize, height: usize) {
        for i in x..x + width {
            for j in y..y + height {
                match image.get_pixel_mut_checked(i as u32, j as u32) {
                    Some(s) => {
                        *s = self.wall_color;
                    }
                    None => {}
                }
            }
        }
    }
}
