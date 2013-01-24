use rand::{prelude::SmallRng, Rng, SeedableRng};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LabyrinthConfig {
    kind: LabyrinthKind,
    /// center size
    center: usize,
    /// rings out of center
    rings: usize,
    /// number of exits
    exits: usize,
    seed: u64,
}

pub enum LabyrinthKind {
    /// The number of forks increases linearly,
    ///
    /// - one for the core
    /// - two for the first circle
    /// - three for the second circle
    /// - and so on
    Linear,
}

impl Default for LabyrinthKind {
    fn default() -> Self {
        LabyrinthKind::Linear
    }
}

impl Default for LabyrinthConfig {
    fn default() -> Self {
        LabyrinthConfig { kind: Default::default(), center: 2, rings: 8, entrances: 1, exits: 1, seed: 0 }
    }
}

impl LabyrinthConfig {
    fn get_angle(&self) -> f32 {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        rng.gen_range(0.0, 2.0 * std::f32::consts::PI)
    }
}
