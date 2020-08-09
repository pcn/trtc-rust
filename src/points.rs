// #[macro_use]
#![warn(unused_imports)]
use std::ops::{Add, Sub, Neg};
use std::cmp::PartialEq;
use crate::vectors::{Vector,vector};

#[derive(TypeName, Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}


pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point {
        x: x,
        y: y,
        z: z,
        w: 1.0,
    }
}

// Point traits
impl Add<&Vector> for &Point {
    type Output = Point;
    fn add(self, other: &Vector) -> Point {
        point(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, other: Vector) -> Point {
        point(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}


impl Default for Point {
    fn default() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point {x: 0.0 - self.x, y: 0.0 - self.y, z: 0.0 - self.z, w: 0.0 - self.w}
    }
}

/// Note: a point can't be added to a point, so that won't be implemented 
/// Note: a point subtracted from a point is a vector

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        vector(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}


impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, other: Vector) -> Point {
        point(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// End Point traits

