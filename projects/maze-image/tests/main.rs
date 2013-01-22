use maze_core::square::Maze2DConfig;
use maze_image::{MazeBlockRenderer, MazeLineRenderer};
#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test2() -> std::io::Result<()> {
    let config = Maze2DConfig::default().with_size(20, 20);
    let out = config.build_dfs().last().unwrap();
    let block = MazeBlockRenderer::new(20);
    let image = block.render_image_2d(&out);
    image.save("block.png").unwrap();
    let line = MazeLineRenderer::new(41).with_wall_width(2);
    let image = line.render_image_2d(&out);
    image.save("line.png").unwrap();
    Ok(())
}

#[test]
#[ignore]
#[cfg(feature = "mota")]
fn test_mota() -> Result<(), image::error::ImageError> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?.join("assets");
    let config = Maze2DConfig::default().with_size(20, 20);
    let out = config.build_dfs().last().unwrap();
    let mut mota = maze_image::MazeMotaRenderer::default();
    mota.add_wall(&here.join("wall.png"))?;
    mota.add_floor(&here.join("floor_up.png"), &here.join("floor_down.png"))?;
    mota.add_road(&here.join("road.png"), 100.0)?;
    mota.add_item(&here.join("door1.png"), 1.0)?;
    mota.add_item(&here.join("item1.png"), 2.0)?;
    mota.add_item(&here.join("item2.png"), 4.0)?;
    mota.add_item(&here.join("enemy1.png"), 8.0)?;
    mota.add_item(&here.join("enemy2.png"), 8.0)?;
    mota.add_item(&here.join("enemy3.png"), 8.0)?;
    let image = mota.render_2d(&out);
    image.save("mota.png").unwrap();
    Ok(())
}
