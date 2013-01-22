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
    pub(crate) fn get_isize(&self, maze: &Maze2D) -> (isize, isize) {
        let (mut w, mut h) = maze.get_size();
        w = w * 2 + 1;
        h = h * 2 + 1;
        w *= self.block_size;
        h *= self.block_size;
        (w as isize, h as isize)
    }
    pub fn render_image_2d(&self, maze: &Maze2D) -> RgbaImage {
        let m01 = maze.matrix01();
        let (w, h) = self.get_isize(maze);
        let mut image = RgbaImage::new(w as u32, h as u32);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let x = x as usize;
            let y = y as usize;
            let x = x / self.block_size;
            let y = y / self.block_size;
            if m01[(x as isize, y as isize)] {
                *pixel = Rgba([0, 0, 0, 0]);
            }
            else {
                *pixel = self.wall_color;
            }
        }
        image
    }
}
