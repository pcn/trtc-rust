// #[macro_use]
// extern crate typename;
#![warn(unused_imports)]
use std::ops::{Add, Sub, Neg, Mul};
use std::cmp::PartialEq;
use crate::equal;
use crate::tuples::{Tuple,ToTuple};
use crate::points::{Point,point};

#[derive(TypeName, Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

// Magnitude is probably only useful for vectors. TODO: can I specialize the default to just vector?
trait Magnitude {
    fn magnitude(&self) -> f64;
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector {
        x: x,
        y: y,
        z: z,
        w: 0.0,
    }
}

// Vector traits and functions

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        point(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, ref other: Vector) -> Vector {
        vector(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Default for Vector {
    fn default() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}

pub fn magnitude(v: &Vector) -> f64 {
    (v.x.powf(2.0) +  v.y.powf(2.0) +  v.z.powf(2.0)).sqrt()
}

pub fn normalize(v: &Vector) -> Vector {
    let mag = magnitude(v);
    vector(v.x / mag, v.y / mag, v.z / mag)
}

pub fn dot(first: &Vector, second: &Vector) -> f64 {
    first.x * second.x +
        first.y * second.y +
        first.z * second.z
}

pub fn cross(first: &Vector, second: &Vector) -> Vector {
    vector(first.y * second.z - first.z * second.y,
           first.z * second.x - first.x * second.z,
           first.x * second.y - first.y * second.x)
}
    


impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, other: f64) -> Self::Output {
        vector(self.x * other, self.y * other, self.z * other)
    }
}
impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector {x: 0.0 - self.x, y: 0.0 - self.y, z: 0.0 - self.z, w: 0.0 - self.w}
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        if ! equal(self.x, other.x) {
            // println!("x failed");
            return false
        };
        if ! equal(self.y, other.y) {
            // println!("y failed");
            return false
        };
        if ! equal(self.z, other.z) {
            // println!("z failed");
            return false
        };
        true
    }
}
        

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        vector(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ToTuple for Vector {
    fn to_tuple(&self) -> Tuple {
        Tuple {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}
// End Vector traits

