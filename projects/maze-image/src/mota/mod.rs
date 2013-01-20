use image::RgbaImage;
use maze_core::{square::Maze2D, Rng, SeedableRng, SmallRng};
use std::path::Path;

pub struct MazeMotaRenderer {
    wall: RgbaImage,
    floor_up: RgbaImage,
    floor_down: RgbaImage,
    items: Vec<(f32, RgbaImage)>,
    road: (f32, RgbaImage),
}

impl Default for MazeMotaRenderer {
    fn default() -> Self {
        Self {
            wall: Default::default(),
            floor_up: Default::default(),
            floor_down: Default::default(),
            items: vec![],
            road: (0.0, Default::default()),
        }
    }
}

impl MazeMotaRenderer {
    pub fn add_wall(&mut self, image: &Path) -> Result<(), image::ImageError> {
        self.wall = image::open(image)?.into_rgba8();
        Ok(())
    }
    pub fn add_floor(&mut self, up: &Path, down: &Path) -> Result<(), image::ImageError> {
        self.floor_up = image::open(up)?.into_rgba8();
        self.floor_down = image::open(down)?.into_rgba8();
        Ok(())
    }
    pub fn add_item(&mut self, image: &Path, weight: f32) -> Result<(), image::ImageError> {
        self.items.push((weight, image::open(image)?.into_rgba8()));
        Ok(())
    }
    pub fn add_road(&mut self, image: &Path, weight: f32) -> Result<(), image::ImageError> {
        self.road = (weight, image::open(image)?.into_rgba8());
        Ok(())
    }
    pub fn render_2d(&self, maze: &Maze2D) -> RgbaImage {
        const CELL_SIZE: usize = 32;
        let mut rng = SmallRng::from_entropy();
        let size = maze.get_size();
        let width = (size.0 * CELL_SIZE) as u32;
        let height = (size.1 * CELL_SIZE) as u32;
        let mut image = RgbaImage::new(width, height);
        for ((i, j), v) in maze.matrix01().indexed_iter() {
            let x = (i * CELL_SIZE) as u32;
            let y = (j * CELL_SIZE) as u32;
            let new = self.select_image(&mut rng, *v);
            for (nx, ny, pixel) in new.enumerate_pixels() {
                let nx = x + nx;
                let ny = y + ny;
                if nx < width && ny < height {
                    image.put_pixel(nx, ny, *pixel);
                }
            }
        }
        image
    }
    // selected in [door, monster, item, road]
    pub fn select_image(&self, rng: &mut SmallRng, is_path: bool) -> &RgbaImage {
        if is_path {
            let weight = rng.gen_range(0.0..self.all_weight());
            let mut sum = 0.0;
            for (w, image) in &self.items {
                sum += *w;
                if weight < sum {
                    return image;
                }
            }
            &self.road.1
        }
        else {
            &self.wall
        }
    }
    fn all_weight(&self) -> f32 {
        self.items.iter().map(|(w, _)| *w).sum::<f32>() + self.road.0
    }
}
