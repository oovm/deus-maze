use deus_maze::square::Maze2DConfig;

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
fn test2() {
    let config = Maze2DConfig::default().with_size(50, 50);
    let out = config.build_dfs().last().unwrap();
    print!("{}", out.render_box_drawings())
}

#[test]
fn test3() {
    let config = Maze2DConfig::default().with_size(20, 20);
    let out = config.build_dfs().last().unwrap();
    svg::save("image.svg", &out.get_svg_rect()).unwrap()
}
