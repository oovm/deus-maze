use std::collections::BTreeSet;
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MazeState {
    pub width: usize,
    pub height: usize,
    pub walked: BTreeSet<(usize, usize)>,
    pub worker: Worker,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Worker {
    walked: Vec<(usize, usize)>,
}

impl MazeState {
    pub fn nearby(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(4);
        if x > 0 {
            result.push((x - 1, y));
        }
        if x < self.width - 1 {
            result.push((x + 1, y));
        }
        if y > 0 {
            result.push((x, y - 1));
        }
        if y < self.height - 1 {
            result.push((x, y + 1));
        }
        result.into_iter().filter(|(x, y)| !self.walked.contains(&(*x, *y))).collect()
    }
}

impl Worker {
    pub fn go_walk(&mut self, x: usize, y: usize, rng: &mut SmallRng) {
        self.walked.push((x, y));
    }
    pub fn go_back(&mut self) {
        self.walked.pop();
    }
}

impl Maze2DConfig {
    pub fn build_path_matrix(&self) -> Array2<bool> {
        todo!()
    }

    pub fn build_grid_matrix(&self) -> Array2<bool> {
        todo!()
    }

    pub fn build_bfs(&self) -> Maze2D {
         let state = MazeState {
            width: self.width,
            height: self.height,
            walked: BTreeSet::new(),
            worker: Worker {
                walked: Vec::new(),
            },
        };
        println!("{:?}", state.nearby(0, 0));
        todo!()
    }
}

#[test]
fn test() {
    let config = Maze2DConfig::default();
    let maze = config.build_bfs();
    println!("{:?}", maze);
}