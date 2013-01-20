use maze_core::square::Maze2DConfig;
use maze_image::{MazeBlockRenderer, MazeLineRenderer};
#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let config = Maze2DConfig::default().with_size(5, 5);
    for (i, maze) in config.build_dfs().enumerate() {
        println!("Maze #{}", i);
        println!("{}", maze);
    }
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
