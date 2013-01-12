use deus_maze::square::Maze2DConfig;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let config = Maze2DConfig::default();
    for (i, maze) in config.build_dfs().enumerate() {
        println!("Maze #{}", i);
        println!("{}", maze);
    }
}
