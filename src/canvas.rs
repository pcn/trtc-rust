#![warn(unused_imports)]

use crate::colors::{Color, color};

// Should I use a vector or a slice?
pub type Canvas = Vec<Vec<Color>>;

pub fn canvas(width: u32, height: u32) -> Canvas {
    let empty_color = color(0.0, 0.0, 0.0);
    vec![vec![empty_color; height as usize]; width as usize]
}
                             
