#[macro_use]
extern crate typename;

pub mod tuples;
pub mod vectors;
pub mod points;
pub mod colors;
pub mod canvas;
pub mod matrix;
pub mod transformations;

pub const EPSILON: f64 = 0.0001;

// A test for "equal enough" for f64 numbers
fn equal(a: f64, b: f64) -> bool {
    f64::abs(a - b) < EPSILON
}

