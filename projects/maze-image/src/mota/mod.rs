use image::RgbaImage;
use maze_core::{square::Maze2D, SeedableRng, SmallRng};
use std::path::Path;

pub struct MazeMotaRenderer {
    wall: RgbaImage,
    doors: Vec<RgbaImage>,
    floor_up: RgbaImage,
    floor_down: RgbaImage,
    monsters: Vec<RgbaImage>,
    items: Vec<RgbaImage>,
    road: RgbaImage,
    door_weight: f32,
    monster_weight: f32,
    item_weight: f32,
    road_weight: f32,
}

impl Default for MazeMotaRenderer {
    fn default() -> Self {
        Self {
            doors: vec![],
            floor_up: Default::default(),
            floor_down: Default::default(),
            road: Default::default(),
            wall: Default::default(),
            monsters: vec![],
            items: vec![],
            door_weight: 0.02,
            item_weight: 0.04,
            monster_weight: 0.08,
            road_weight: 1.0,
        }
    }
}

impl MazeMotaRenderer {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn set_wall(&mut self, image: &Path) -> Result<(), image::ImageError> {
        self.wall = image::open(image)?.to_rgba();
        Ok(())
    }
    pub fn set_floor_up(&mut self, up: &Path, down: &Path) -> Result<(), image::ImageError> {
        self.floor_up = image::open(up)?.to_rgba();
        self.floor_down = image::open(down)?.to_rgba();
        Ok(())
    }

    pub fn add_door(&mut self, image: &Path) -> Result<(), image::ImageError> {
        self.doors.push(image::open(image)?.to_rgba());
        Ok(())
    }
    pub fn set_door_weight(&mut self, weight: f32) {
        self.door_weight = weight;
    }
    pub fn add_monster(&mut self, image: &Path) -> Result<(), image::ImageError> {
        self.monsters.push(image::open(image)?.to_rgba());
        Ok(())
    }
    pub fn set_monster_weight(&mut self, weight: f32) {
        self.monster_weight = weight;
    }
    pub fn add_item(&mut self, image: &Path) -> Result<(), image::ImageError> {
        self.items.push(image::open(image)?.to_rgba());
        Ok(())
    }
    pub fn set_item_weight(&mut self, weight: f32) {
        self.item_weight = weight;
    }
    pub fn render_2d(&self, maze: &Maze2D) -> RgbaImage {
        const CELL_SIZE: usize = 32;
        let mut rng = SmallRng::from_entropy();
        let width = (maze.ncols() * CELL_SIZE) as u32;
        let height = (maze.nrows() * CELL_SIZE) as u32;
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
            let mut r = rng.gen_range(0.0, self.all_weight());
            // maybe road
            if r < self.road_weight {
                return &self.road;
            }
            r -= self.road_weight;
            // maybe monster
            if r < self.monster_weight {
                return &self.monsters[rng.gen_range(0, self.monsters.len())];
            }
            r -= self.monster_weight;
            // maybe item
            if r < self.item_weight {
                return &self.items[rng.gen_range(0, self.items.len())];
            }
            r -= self.item_weight;
            // maybe door
            &self.doors[rng.gen_range(0, self.doors.len())]
        }
        else {
            &self.wall
        }
    }
    fn all_weight(&self) -> f32 {
        self.door_weight + self.monster_weight + self.item_weight + self.road_weight
    }
}
