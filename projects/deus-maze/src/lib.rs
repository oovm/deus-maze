#![feature(generators)]
#![feature(iter_from_generator)]

mod errors;

pub use errors::{Error, Result};

mod renderer;
pub mod square;
