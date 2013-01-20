#![feature(generators)]
#![feature(iter_from_generator)]

pub mod hexagon;
pub mod square;

pub use rand::{rngs::SmallRng, SeedableRng};
