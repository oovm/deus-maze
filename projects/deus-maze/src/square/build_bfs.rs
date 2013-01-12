use super::*;
use rand::Rng;
use std::{collections::BTreeSet, iter::from_generator};

#[derive(Clone, Debug)]
pub struct MazeState {
    pub width: usize,
    pub height: usize,
    pub walked: BTreeSet<(usize, usize)>,
    pub joints: Vec<Joint>,
    pub rng: SmallRng,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BfsWorker {
    walked: Vec<(usize, usize)>,
}

impl MazeState {
    pub fn nearby(&self, x: usize, y: usize) -> Vec<Joint> {
        Joint::all(x, y, self.width, self.height).iter().filter(|joint| !self.is_walked(joint)).cloned().collect()
    }
    #[inline]
    pub fn is_finished(&self) -> bool {
        self.walked.len() == self.width * self.height
    }
    #[inline]
    pub fn is_walked(&self, joint: &Joint) -> bool {
        self.walked.contains(&joint.target())
    }
}

impl BfsWorker {
    pub fn go_next(&mut self, state: &mut MazeState) {
        match self.walked.last() {
            Some(head) => {
                let mut nearby = state.nearby(head.0, head.1);
                if nearby.is_empty() { self.go_back() } else { self.go_walk(&mut nearby, state) }
            }
            None => {
                todo!()
            }
        }
    }
    pub fn go_back(&mut self) {
        self.walked.pop();
    }
    pub fn go_walk(&mut self, nearby: &[Joint], state: &mut MazeState) {
        let index = state.rng.gen_range(0..nearby.len());
        let next = &nearby[index];
        self.walked.push(next.target());
        state.walked.insert(next.target());
        state.joints.push(*next);
    }
}

impl Maze2DConfig {
    pub fn build_path_matrix(&self) -> Array2<bool> {
        todo!()
    }

    pub fn build_grid_matrix(&self) -> Array2<bool> {
        todo!()
    }
    pub fn build_dfs(&self) -> impl Iterator<Item = Maze2D> {
        let config = self.clone();
        let mut worker = BfsWorker { walked: vec![self.get_entry()] };
        let mut state = MazeState {
            width: self.width,
            height: self.height,
            walked: BTreeSet::new(),
            joints: Vec::with_capacity(self.width * self.height * 2),
            rng: self.get_rng(),
        };
        from_generator(move || {
            while !state.is_finished() {
                worker.go_next(&mut state);
                yield Maze2D { config: config.clone(), joints: state.joints.clone(), rooms: vec![] };
            }
            yield Maze2D { config: config.clone(), joints: state.joints.clone(), rooms: vec![] };
        })
    }
}
