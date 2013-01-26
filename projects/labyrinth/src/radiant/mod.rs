use rand::{prelude::SmallRng, Rng, SeedableRng};
use std::{
    collections::{BTreeMap, HashSet},
    ops::Range,
};

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
    /// Each turn turns through `δ°` to prevent obvious axis
    delta_angle: f64,
    seed: u64,
}
#[derive(Clone, Debug)]
pub enum LabyrinthKind {
    /// The room number increases linearly,
    ///
    /// - one for the core
    /// - two for the first circle
    /// - three for the second circle
    /// - and so on
    Linear,
    /// The room number increases exponentially,
    ///
    /// Every x circles, the number of rooms is multiplied to x * y
    Multiply(u32, u32),
}

impl LabyrinthKind {
    pub fn as_adjacent(&self, rings: u32, delta: f64) -> HashSet<Joint> {
        match self {
            LabyrinthKind::Linear => LabyrinthKind::linear_adjacent(rings, delta),
            LabyrinthKind::Multiply(_, _) => {
                todo!()
            }
        }
    }
    fn linear_adjacent(rings: u32, delta: f64) -> HashSet<Joint> {
        let mut adjacent = HashSet::new();
        let mut rooms = 0;
        for i in 1..=rings {
            for j in 0..rooms {
                // all rooms in same ring are adjacent
                adjacent.insert(Joint { a: (i, j), b: (i, j + 1) });
                //
                let d = 360.0 / (i as f64);
                let s = (i as f64 * delta) % 360.0;
                let e = (i as f64 * delta + d) % 360.0;
            }
            rooms += 1;
        }
        adjacent
    }
}

/// A completed labyrinth
pub struct Labyrinth {
    config: LabyrinthConfig,
}

/// A uncompleted labyrinth
pub struct LabyrinthState {
    adjacent: HashSet<Joint>,
}

#[derive(Copy, Clone)]
pub struct Segment360 {
    head: f64,
    tail: f64,
}

impl Segment360 {
    pub fn normalize(&mut self) {
        if self.head > self.tail {
            self.head -= 360.0;
        }
    }
    pub fn overlaps(&self, other: Segment360) -> Option<Range<f64>> {
        if self.head < other.head {
            if self.tail < other.head {
                None
            }
            else if self.tail < other.tail {
                Some(other.head..self.tail)
            }
            else {
                Some(other.head..other.tail)
            }
        }
        else if self.head < other.tail {
            if self.tail < other.tail { Some(self.head..self.tail) } else { Some(self.head..other.tail) }
        }
        else {
            None
        }
    }
}

fn test() {
    let x = Segment360 { head: 100.0, tail: 200.0 };
    let y = Segment360 { head: 150.0, tail: 250.0 };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Joint {
    a: (u32, u32),
    b: (u32, u32),
}

impl Default for LabyrinthKind {
    fn default() -> Self {
        LabyrinthKind::Linear
    }
}

impl Default for LabyrinthConfig {
    fn default() -> Self {
        LabyrinthConfig { kind: Default::default(), center: 2, rings: 8, exits: 1, delta_angle: 0.0, seed: 0 }
    }
}

impl LabyrinthConfig {
    fn delta_angle(&self) -> f32 {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        rng.gen_range(0.0..2.0 * std::f32::consts::PI)
    }
}
