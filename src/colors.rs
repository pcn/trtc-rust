// #[macro_use]
// extern crate typename;
#![warn(unused_imports)]
use std::ops::{Add, Sub, Neg, Mul, Div};
use std::cmp::PartialEq;
use equal;
use tuples::{Tuple,ToTuple};

#[derive(TypeName, Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color {
        red: r,
        green: g,
        blue: b,
    }
}


// Color traits
impl Default for Color {
    fn default() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

