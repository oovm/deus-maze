use super::*;

#[derive(Clone, Debug)]
pub struct MazeState {
    pub width: usize,
    pub height: usize,
    pub walked: Array2<bool>,
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
        self.walked.iter().all(|&walked| walked)
    }
    #[inline]
    pub fn is_walked(&self, joint: &Joint) -> bool {
        let (x, y) = joint.target();
        self.walked[[x, y]]
    }
}

impl BfsWorker {
    pub fn go_back(&mut self) {
        self.walked.pop();
    }
    pub fn go_walk(&mut self, nearby: &[Joint], state: &mut MazeState) {
        let index = state.rng.gen_range(0..nearby.len());
        let joint = &nearby[index];
        let (x, y) = (joint.target().0, joint.target().1);
        self.walked.push((x, y));
        state.walked[[x, y]] = true;
        state.joints.push(*joint);
    }
}

impl Maze2DConfig {
    pub fn initial(&self) -> Array2<bool> {
        let mut walked = Array2::from_elem((self.width, self.height), false);
        let (x, y) = self.get_entry();
        walked[[x, y]] = true;
        for (x, y) in self.bad.iter() {
            walked[[*x, *y]] = true;
        }
        walked
    }
    pub fn build_dfs(&self) -> impl Iterator<Item = Maze2D> {
        let config = self.clone();
        let mut worker = BfsWorker { walked: vec![self.get_entry()] };
        let mut state = MazeState {
            width: self.width,
            height: self.height,
            walked: self.initial(),
            joints: Vec::with_capacity(self.width * self.height * 2),
            rng: self.get_rng(),
        };
        from_generator(move || {
            while !state.is_finished() {
                match worker.walked.last() {
                    Some(head) => {
                        let mut nearby = state.nearby(head.0, head.1);
                        if nearby.is_empty() {
                            worker.go_back()
                        }
                        else {
                            worker.go_walk(&mut nearby, &mut state);
                            yield Maze2D::new(&config, &state.joints, &[]);
                        }
                    }
                    None => {
                        todo!()
                    }
                }
            }
            yield Maze2D::new(&config, &state.joints, &[])
        })
    }
}
