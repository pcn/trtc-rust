#![warn(unused_imports)]
use std::ops::{Neg, Mul, Div};
use std::cmp::PartialEq;
// use equal;


#[derive(TypeName, Debug, PartialEq, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub trait ToTuple {
    // Can't specialize on struct members, I don't think
    fn to_tuple(&self) -> Tuple;
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: w,
    }
}


// Tuple traits
impl Default for Tuple {
    fn default() -> Tuple {
        Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}
// Putting the trait on the f64. Wow. 
impl Div<Tuple> for f64 {
    type Output = Tuple;
    fn div(self, rhs: Tuple) -> Self::Output {
        tuple(rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}
impl Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f64) -> Self::Output {
        rhs / self
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;
    fn mul(self, other: Tuple) -> Self::Output {
        tuple(self * other.x, self * other.y, self * other.z, self * other.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}


impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        Tuple {x: 0.0 - self.x, y: 0.0 - self.y, z: 0.0 - self.z, w: 0.0 - self.w}
    }
}


