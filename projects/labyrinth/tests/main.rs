// use maze_core::square::Maze2DConfig;

use rand::{rngs::StdRng, Rng, SeedableRng};

#[test]
fn ready() {
    println!("it works!")
}
// #[test]
// fn test() {
//     let config = Maze2DConfig::default().with_size(5, 5);
//     for (i, maze) in config.build_dfs().enumerate() {
//         println!("Maze #{}", i);
//         println!("{}", maze);
//     }
// }

pub struct DiceCount {
    all: u32,
    target: u32,
}

impl DiceCount {
    pub fn experience(&self, rng: &mut StdRng) -> f32 {
        let mut sum = 0;
        loop {
            let roll = rng.gen_range(1..=self.all);
            sum += roll;
            if roll == self.target {
                return sum as f32;
            }
        }
    }
}

#[test]
fn test_mean() {
    let mut rng = StdRng::from_entropy();
    let mut sum = 0.0;
    let count = 1000000;
    for _ in 0..count {
        let dice = DiceCount { all: 20, target: 20 };
        sum += dice.experience(&mut rng);
    }
    println!("mean: {}", sum / count as f32);
}
