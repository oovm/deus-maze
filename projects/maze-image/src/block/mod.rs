use image::{Rgba, RgbaImage};
use maze_core::square::Maze2D;

pub struct MazeBlockRenderer {
    block_size: usize,
    wall_color: Rgba<u8>,
}

impl Default for MazeBlockRenderer {
    fn default() -> Self {
        Self { block_size: 10, wall_color: Rgba([0, 0, 0, 255]) }
    }
}

impl MazeBlockRenderer {
    pub fn new(size: usize) -> Self {
        Self { block_size: size, ..Default::default() }
    }
    pub fn render_image_2d(&self, maze: &Maze2D) -> RgbaImage {
        let m01 = maze.matrix01();
        let width = self.block_size * m01.ncols();
        let height = self.block_size * m01.nrows();
        let mut image = RgbaImage::new(width as u32, height as u32);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let x = x as usize;
            let y = y as usize;
            let x = x / self.block_size;
            let y = y / self.block_size;
            if m01[[x, y]] {
                *pixel = Rgba([0, 0, 0, 0]);
            }
            else {
                *pixel = self.wall_color;
            }
        }
        image
    }
}
