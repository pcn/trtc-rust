// #![feature(core_intrinsics)]  // for type_of
extern crate cucumber;
extern crate typename;

use cucumber::{after, before, cucumber, steps};
use typename::TypeName;

extern crate trtc;
use trtc::vectors::Vector;
use trtc::points::Point;
use trtc::tuples::Tuple;

#[derive(TypeName)]
pub struct MyWorld {
    // this struct contains mutable context
    apoint: Point,
    apoint2: Point,
    bvec: Vector,
    bvec2: Vector,
    ctup: Tuple,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a scenario is  started
        MyWorld {
            apoint: Point {
                ..Default::default()
            },
            apoint2: Point {
                ..Default::default()
            },
            bvec: Vector {
                ..Default::default()
            },
            bvec2: Vector {
                ..Default::default()
            },
            ctup: Tuple {
                ..Default::default()
            },
        }
    }
}

mod tuples_steps {
    // Any type that implements cucumber_rust::World + Default can be the world
    use cucumber::steps;
    use typename::TypeName;
    use trtc::vectors::{Vector, vector, magnitude, normalize, dot, cross};
    use trtc::points::{Point, point};
    use trtc::tuples::{Tuple, tuple};
    steps!(::MyWorld => {
        // Scenario: A tuple with w=1 is a point
        given regex r#"a is a tuple of ([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)"# (f64, f64, f64, f64) | world, x, y , z, w, _step| {
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            world.apoint = Point{x: x, y: y, z: z, w: w}
        };
        then regex r#"a.x = ([-0-9.]+)"# (f64) | world, x, _step| {
            assert_eq!(world.apoint.x, x)
        };
        then regex r#"a.y = ([-0-9.]+)"# (f64) | world, y, _step| {
            assert_eq!(world.apoint.y, y)
        };
        then regex r#"a.z = ([-0-9.]+)"# (f64) | world, z, _step| {
            assert_eq!(world.apoint.z, z)
        };
        then regex r#"a.w = ([-0-9.]+)"# (f64) | world, w, _step| {
            assert_eq!(world.apoint.w,  w)
        };
        then r#"a is a point"# | world, _step| {
            // assert_eq!(::type_of(&world.a), "trtc::tuples::Point")
            assert_eq!(world.apoint.type_name_of(), "trtc::points::Point");
        };
        then r#"a is not a vector"# | world, _step| {
            // assert!(::type_of(&world.a) != "trtc::tuples::Vector")
            assert!(world.apoint.type_name_of() != "trtc::vectors::Vector")
        };

        // Scenario: A tuple with w=0 is a vector
        given regex r#"b is a tuple of ([-0-9.]+), ([-0-9.]+), ([-0-9.]+), ([-0-9.]+)"# (f64, f64, f64, f64) | world, x, y , z, w, _step| {
            world.bvec = Vector{x: x, y: y, z: z, w: w}
        };
        then regex r#"b.x = ([-0-9.]+)"# (f64) | world, x, _step| {
            assert_eq!(world.bvec.x, x)
        };
        then regex r#"b.y = ([-0-9.]+)"# (f64) | world, y, _step| {
            assert_eq!(world.bvec.y, y)
        };
        then regex r#"b.z = ([-0-9.]+)"# (f64) | world, z, _step| {
            assert_eq!(world.bvec.z, z)
        };
        then regex r#"b.w = ([-0-9.]+)"# (f64) | world, w, _step| {
            assert_eq!(world.bvec.w,  w)
        };
        then r#"b is a vector"# | world, _step| {
            // assert_eq!(::type_of(&world.b), "trtc::tuples::Vector")
            assert_eq!(world.bvec.type_name_of(), "trtc::vectors::Vector")
        };
        then r#"b is not a point"# | world, _step| {
            // assert!(::type_of(&world.b) != "trtc::tuples::Point")
            assert!(world.bvec.type_name_of() != "trtc::points::Point")
        };

        // // Scenario: "Point" describes tuples with w=1
        // given regex r#"Point describes a tuple with type ([A-Za-z0-9:]+) and w=1# (str) | world, t_type, _step| {
        //     assert!(world.a.type_name_of() == trtc::tuples::Vector{x: x, y: y, z: z, w: w})
        // };

        given regex r#"p = point\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.apoint = point(x, y, z)
        };

        //   Scenario: Adding two tuples
        given regex r#"a1 <- point\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.apoint = point(x, y, z)
        };

        given regex r#"a2 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };

        then regex r#"a1 \+ a2 == point\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            assert!((world.apoint + world.bvec) == point(x, y, z))
        };

        // Scenario: Subtracting two points
        given regex r#"p1 <- point\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.apoint = point(x, y, z)
        };
        given regex r#"p2 <- point\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.apoint2 = point(x, y, z)
        };
        then regex r#"p1 - p2 == vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            assert!(world.apoint - world.apoint2 == vector(x, y, z))
        };
        
        // Scenario: Subtracting a vector from a point
        given regex r#"p <- point\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.apoint = point(x, y, z)
        };
        given regex r#"v <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.bvec = vector(x, y, z)
        };
        then regex r#"p - v == point\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            // println!("{:?}", world.apoint - world.bvec);
            assert!(world.apoint - world.bvec == point(x, y, z))
        };
        
        // Scenario: Subtracting two vectors
        given regex r#"v1 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.bvec = vector(x, y, z)
        };
        given regex r#"v2 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.bvec2 = vector(x, y, z)
        };
        then regex r#"v1 - v2 == vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            assert!(world.bvec - world.bvec2 == vector(x, y, z))
        };
        
        // Scenario: Subracting a vector from the zero vector
        given regex r#"zero <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.bvec = vector(x, y, z)
        };
        given regex r#"v_sub <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.bvec2 = vector(x, y, z)
        };
        then regex r#"zero - v == vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            assert!(world.bvec - world.bvec2 == vector(x, y, z))
        };

        // Scenario: Negating a tuple
        given regex r#"c <- tuple\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64, f64) | world, x, y, z, w, _step| {
            world.ctup = tuple(x, y, z, w)
        };
        then regex r#"-c == tuple\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64, f64) | world, x, y, z, w, _step| {
            assert!(-world.ctup == tuple(x, y, z, w))
        };
        // Scenario: Negating a vector
        given regex r#"bneg <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            world.bvec = vector(x, y, z)
        };
        then regex r#"-bneg == vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step| {
            assert!(-world.bvec == vector(x, y, z))
        };
        // Scenario: Multiplying a tuple by a scalar
        given regex r#"Given c <- tuple\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64, f64) | world, x, y, z, w, _step| {
            world.ctup = tuple(x, y, z, w)
        };
        then regex r#"c \* ([\-0-9.]+) == tuple\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64, f64, f64) | world, mul, x, y, z, w, _step| {
            assert!(mul * world.ctup == tuple(x, y, z, w))
        };
        // Scenario: Multiplying a tuple by a fraction
        given regex r#"Given c_frac <- tuple\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64, f64) | world, x, y, z, w, _step| {
            world.ctup = tuple(x, y, z, w)
        };
        then regex r#"c_frac \* ([\-0-9.]+) == tuple\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64, f64, f64) | world, mul, x, y, z, w, _step| {
            assert!(mul * world.ctup == tuple(x, y, z, w))
        };
        // Scenario: Dividing a tuple by a scaler
        given regex r#"Given c_div <- tuple\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64, f64) | world, x, y, z, w, _step| {
            world.ctup = trtc::tuples::tuple(x, y, z, w)
        };
        then regex r#"c_div \* ([\-0-9.]+) == tuple\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64, f64, f64) | world, div, x, y, z, w, _step| {
            assert!(world.ctup/div == tuple(x, y, z, w))
        };

        // Scenario: Computing the magnitude of vector(1, 0, 0)
        given regex r#"v100 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.])\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        then regex r#"magnitude\(v100\) == ([\-0-9.]+)"# (f64) | world, mag, _step | {
            assert!(magnitude(&world.bvec) == mag)
        };
        // Scenario: Computing the magnitude of vector(0, 1, 0)
        given regex r#"v010 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.])\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        then regex r#"magnitude\(v010\) == ([\-0-9.]+)"# (f64) | world, mag, _step | {
            assert!(magnitude(&world.bvec) == mag)
        };
        // Scenario: Computing the magnitude of vector(0, 0, 1)
        given regex r#"v001 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.])\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        then regex r#"magnitude\(v001\) == ([\-0-9.]+)"# (f64) | world, mag, _step | {
            assert!(magnitude(&world.bvec) == mag)
        };
        // Scenario: Computing the magnitude of vector(1, 2, 3)
        given regex r#"v123 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.])\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        then regex r#"magnitude\(v123\) == sqrt\(([\-0-9.]+)\)"# (f64) | world, mag, _step | {
            assert!(magnitude(&world.bvec) == mag.sqrt())
        };
        // Scenario: Computing the magnitude of vector(-1, -2, -3)
        given regex r#"vneg123 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.])\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        then regex r#"magnitude\(vneg123\) == sqrt\(([\-0-9.]+)\)"# (f64) | world, mag, _step | {
            assert!(magnitude(&world.bvec) == mag.sqrt())
        };
        // Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
        given regex r#"vnorm <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        then regex r#"normalize\(vnorm\) == vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            assert!(normalize(&world.bvec) == vector(x, y , z))
        };
        // Scenario: Normalizing vector(1, 2, 3)
        given regex r#"vnorm123 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        then regex r#"normalize\(vnorm123\) == approximately vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            assert!(normalize(&world.bvec) == vector(x, y , z))
        };
        // Scenario: The magnitude of a normalized vector
        given regex r#"vnorm2 <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        when r#"norm <- normalize(vnorm2)"# | world, _step | {
            world.bvec2 = normalize(&world.bvec)
        };
        then r#"magnitude(norm) == 1"# | world, _step | {
            assert!(magnitude(&world.bvec2) == 1.0)
        };
        // Scenario: The dot product of two tuples
        given regex r#"tadot <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        given regex r#"tbdot <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec2 = vector(x, y, z)
        };
        then regex r#"dot\(tadot, tbdot\) == ([\-0-9.]+)"# (f64) | world, dotp, _step | {
            assert!(dot(&world.bvec, &world.bvec2) == 20.0)
        };

        // Scenario: the cross product of two vectors
        given regex r#"across <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec = vector(x, y, z)
        };
        given regex r#"bcross <- vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            world.bvec2 = vector(x, y, z)
        };
        then regex r#"cross\(across, bcross\) == vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            assert_eq!(cross(&world.bvec, &world.bvec2), vector(x, y, z))
        };
        then regex r#"cross\(bcross, across\) == vector\(([\-0-9.]+), ([\-0-9.]+), ([\-0-9.]+)\)"# (f64, f64, f64) | world, x, y, z, _step | {
            assert_eq!(cross(&world.bvec2, &world.bvec), vector(x, y, z))
        };
    });
    
}

cucumber! {
    features: "./features", // Path to our feature files
    world: ::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        tuples_steps::steps // the `steps!` macro creates a `steps` function in a module
    ]
    // setup: setup, // Optional; called once before everything
    // before: &[
    //     a_before_fn // Optional; called before each scenario
    // ],
    // after: &[
    //     an_after_fn // Optional; called after each scenario
    // ls]
}
