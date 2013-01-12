use deus_maze::square::Maze2DConfig;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let config = Maze2DConfig::default();
    let maze = config.build_dfs().last().unwrap();
    println!("{}", maze);
}
