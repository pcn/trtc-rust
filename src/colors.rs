// #[macro_use]
// extern crate typename;
#![warn(unused_imports)]
use std::ops::{Add, Sub, Neg, Mul, Div};
use std::cmp::PartialEq;
use crate::equal;
// use crate::tuples::{Tuple,ToTuple};

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

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, ref other: Color) -> Color {
        color(self.red + other.red,
              self.green + other.green,
              self.blue + other.blue)
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, ref other: Color) -> Color {
        color(self.red - other.red,
              self.green - other.green,
              self.blue - other.blue)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    // According to the book, this method of blending colors
    // is creating the hadamard product or the schur product
    fn mul(self, ref other: Color) -> Color {
        color(self.red * other.red,
              self.green * other.green,
              self.blue * other.blue)
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    // According to the book, this method of blending colors
    // is creating the hadamard product or the schur product
    fn mul(self, ref other: f64) -> Color {
        color(self.red * other,
              self.green * other,
              self.blue * other)
    }
}


impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        if ! equal(self.red, other.red) {
            // println!("x failed");
            return false
        };
        if ! equal(self.blue, other.blue) {
            // println!("y failed");
            return false
        };
        if ! equal(self.green, other.green) {
            // println!("z failed");
            return false
        };
        true
    }
}
