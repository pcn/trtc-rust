#[macro_use]
extern crate typename;

pub mod tuples;
pub mod vectors;
pub mod points;
pub mod colors;

const EPSILON: f64 = 0.00001;

// A test for "equal enough" for f64 numbers
fn equal(a: f64, b: f64) -> bool {
    f64::abs(a - b) < EPSILON
}

// #[macro_use]
// extern crate typename;

// const EPSILON: f64 = 0.00001;

// // A test for "equal enough" for f64 numbers
// fn equal(a: f64, b: f64) -> bool {
//     f64::abs(a - b) < EPSILON
// }

// pub mod tuples {
//     use std::ops::{Add, Sub, Neg, Mul, Div};
//     use std::cmp::PartialEq;
//     use ::equal;

//     #[derive(TypeName, Debug, PartialEq)]
//     pub struct Point {
//         pub x: f64,
//         pub y: f64,
//         pub z: f64,
//         pub w: f64,
//     }
 
//     #[derive(TypeName, Debug)]
//     pub struct Vector {
//         pub x: f64,
//         pub y: f64,
//         pub z: f64,
//         pub w: f64,
//     }

//     #[derive(TypeName, Debug, PartialEq)]
//     pub struct Tuple {
//         pub x: f64,
//         pub y: f64,
//         pub z: f64,
//         pub w: f64,
//     }

//     trait ToTuple {
//         // Can't specialize on struct members, I don't think
//         fn to_tuple(&self) -> Tuple;
//     }

//     // Magnitude is probably only useful for vectors. TODO: can I specialize the default to just vector?
//     trait Magnitude {
//         fn magnitude(&self) -> f64;
//     }

//     pub fn point(x: f64, y: f64, z: f64) -> Point {
//         Point {
//             x: x,
//             y: y,
//             z: z,
//             w: 1.0,
//         }
//     }

//     pub fn vector(x: f64, y: f64, z: f64) -> Vector {
//         Vector {
//             x: x,
//             y: y,
//             z: z,
//             w: 0.0,
//         }
//     }

//     pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
//         Tuple {
//             x: x,
//             y: y,
//             z: z,
//             w: w,
//         }
//     }

//     // Point traits
//     impl Add<&Vector> for &Point {
//         type Output = Point;
//         fn add(self, other: &Vector) -> Point {
//             point(self.x + other.x, self.y + other.y, self.z + other.z)
//         }
//     }

//     impl Default for Point {
//         fn default() -> Point {
//             Point {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 0.0,
//                 w: 1.0,
//             }
//         }
//     }
    
//     impl Neg for &Point {
//         type Output = Point;
//         fn neg(self) -> Self::Output {
//             Point {x: 0.0 - self.x, y: 0.0 - self.y, z: 0.0 - self.z, w: 0.0 - self.w}
//         }
//     }
    
//     /// Note: a point can't be added to a point, so that won't be implemented 
//     /// Note: a point subtracted from a point is a vector
//     impl Sub<&Point> for &Point {
//         type Output = Vector;
//         fn sub(self, other: &Point) -> Vector {
//             vector(self.x - other.x, self.y - other.y, self.z - other.z)
//         }
//     }

//     impl Sub<&Vector> for &Point {
//         type Output = Point;
//         fn sub(self, other: &Vector) -> Point {
//             point(self.x - other.x, self.y - other.y, self.z - other.z)
//         }
//     }

//     // End Point traits
    
//     // Vector traits and functions
//     impl Add<&Point> for &Vector {
//         type Output = Point;
//         fn add(self, other: &Point) -> Point {
//             point(self.x + other.x, self.y + other.y, self.z + other.z)
//         }
//     }
    
//     impl Add<&Vector> for &Vector {
//         type Output = Vector;
//         fn add(self, ref other: &Vector) -> Vector {
//             vector(self.x + other.x, self.y + other.y, self.z + other.z)
//         }
//     }
    
//     impl Default for Vector {
//         fn default() -> Vector {
//             Vector {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 0.0,
//                 w: 0.0,
//             }
//         }
//     }

//     pub fn magnitude(v: &Vector) -> f64 {
//         (v.x.powf(2.0) +  v.y.powf(2.0) +  v.z.powf(2.0)).sqrt()
//     }
//     pub fn normalize(v: &Vector) -> Vector {
//         let mag = magnitude(v);
//         vector(v.x / mag, v.y / mag, v.z / mag)
//     }
            

//     impl Mul<f64> for &Vector {
//         type Output = Vector;
//         fn mul(self, other: f64) -> Self::Output {
//             vector(self.x * other, self.y * other, self.z * other)
//         }
//     }
//     impl Neg for &Vector {
//         type Output = Vector;
//         fn neg(self) -> Self::Output {
//             Vector {x: 0.0 - self.x, y: 0.0 - self.y, z: 0.0 - self.z, w: 0.0 - self.w}
//         }
//     }

//     impl PartialEq for Vector {
//         fn eq(&self, other: &Vector) -> bool {
//             if ! equal(self.x, other.x) {
//                 // println!("x failed");
//                 return false
//             };
//             if ! equal(self.y, other.y) {
//                 // println!("y failed");
//                 return false
//             };
//             if ! equal(self.z, other.z) {
//                 // println!("z failed");
//                 return false
//             };
//             true
//         }
//     }
            

//     impl Sub<&Vector> for &Vector {
//         type Output = Vector;
//         fn sub(self, other: &Vector) -> Vector {
//             vector(self.x - other.x, self.y - other.y, self.z - other.z)
//         }
//     }
    
//     impl ToTuple for Vector {
//         fn to_tuple(&self) -> Tuple {
//             Tuple {
//                 x: self.x,
//                 y: self.y,
//                 z: self.z,
//                 w: self.w,
//             }
//         }
//     }
//     // End Vector traits

//     // Tuple traits
//     impl Default for Tuple {
//         fn default() -> Tuple {
//             Tuple {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 0.0,
//                 w: 0.0,
//             }
//         }
//     }
//     // Putting the trait on the f64. Wow. 
//     impl Div<&Tuple> for f64 {
//         type Output = Tuple;
//         fn div(self, rhs: &Tuple) -> Self::Output {
//             tuple(rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
//         }
//     }
//     impl Div<f64> for &Tuple {
//         type Output = Tuple;
//         fn div(self, rhs: f64) -> Self::Output {
//             rhs / self
//         }
//     }

//     impl Mul<&Tuple> for f64 {
//         type Output = Tuple;
//         fn mul(self, other: &Tuple) -> Self::Output {
//             tuple(self * other.x, self * other.y, self * other.z, self * other.w)
//         }
//     }

//     impl Mul<f64> for &Tuple {
//         type Output = Tuple;
//         fn mul(self, rhs: f64) -> Self::Output {
//             rhs * self
//         }
//     }
    
    
//     impl Neg for &Tuple {
//         type Output = Tuple;
//         fn neg(self) -> Self::Output {
//             Tuple {x: 0.0 - self.x, y: 0.0 - self.y, z: 0.0 - self.z, w: 0.0 - self.w}
//         }
//     }
//     // End Tuple traits
    
// }
