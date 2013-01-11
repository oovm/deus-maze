mod errors;

pub use errors::{Error, Result};

mod square;
mod renderer;

pub use crate::square::{Maze2DConfig, Maze2D};