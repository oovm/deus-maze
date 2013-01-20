use image::RgbaImage;

pub struct ItemRenderer {
    image: RgbaImage,
}

pub enum ItemKind {
    Wall,
    Road,
    FloorUp,
    FloorDown,
}
