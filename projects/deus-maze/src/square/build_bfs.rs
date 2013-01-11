use std::collections::BTreeSet;
use rand::Rng;
use super::*;

#[derive(Clone, Debug)]
pub struct MazeState {
    pub width: usize,
    pub height: usize,
    pub walked: BTreeSet<(usize, usize)>,
    pub joints: BTreeSet<Joint>,
    pub rng: SmallRng,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Joint {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Joint {
    pub fn all(x: usize, y: usize) -> [Self; 4] {
        [
            Self {
                x,
                y,
                direction: Direction::Up,
            },
            Self {
                x,
                y,
                direction: Direction::Down,
            },
            Self {
                x,
                y,
                direction: Direction::Left,
            },
            Self {
                x,
                y,
                direction: Direction::Right,
            },
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BfsWorker {
    walked: Vec<(usize, usize)>,
}

impl MazeState {
    pub fn nearby(&self, x: usize, y: usize) -> Vec<Direction> {
        let mut result = Vec::with_capacity(4);
        if x > 0 {
            result.push(Direction::Left);
        }
        if x < self.width - 1 {
            result.push(Direction::Right);
        }
        if y > 0 {
            result.push(Direction::Up);
        }
        if y < self.height - 1 {
            result.push(Direction::Down);
        }
        result.into_iter().filter(|(x, y)| !self.is_walked(*x, *y)).collect()
    }
    #[inline]
    pub fn is_finished(&self) -> bool {
        self.walked.len() == self.width * self.height
    }
    #[inline]
    pub fn is_walked(&self, from: usize, direction: Direction) -> bool {
        let (x, y) = match direction {
            Direction::Up => (from.0, from.1 - 1),
            Direction::Down => (from.0, from.1 + 1),
            Direction::Left => (from.0 - 1, from.1),
            Direction::Right => (from.0 + 1, from.1),
        };
        self.walked.contains(&(x, y))
    }
}

impl BfsWorker {
    pub fn go_next(&mut self, state: &mut MazeState) {
        match self.walked.last() {
            Some(head) => {
                let mut nearby = state.nearby(head.0, head.1);
                if nearby.is_empty() {
                    self.go_back()
                } else {
                    self.go_walk(&mut nearby, state)
                }
            }
            None => {
                todo!()
            }
        }
    }
    pub fn go_back(&mut self) {
        self.walked.pop();
    }
    pub fn go_walk(&mut self, nearby: &[(usize, usize)], state: &mut MazeState) {
        let index = state.rng.gen_range(0..nearby.len());
        let next = nearby[index];
        self.walked.push(next);
        state.walked.insert(next);
        state.joints.insert(Joint {
            x: next.0,
            y: next.1,
            direction: index,
        });
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
        let mut worker = BfsWorker {
            walked: vec![self.get_entry()],
        };
        let mut state = MazeState {
            width: self.width,
            height: self.height,
            walked: BTreeSet::new(),
            rng: self.get_rng(),
        };
        while !state.is_finished() {
            worker.go_next(&mut state);
        }
        println!("{:?}", state);
        todo!()
    }
}

#[test]
fn test() {
    let config = Maze2DConfig::default();
    let maze = config.build_bfs();
    println!("{:?}", maze);
}