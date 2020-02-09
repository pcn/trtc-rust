// #![feature(core_intrinsics)]  // for type_of
// extern crate cucumber;
// extern crate typename;
use cucumber::{cucumber};
use typename::{TypeName};

extern crate trtc;
use trtc::colors::{Color}; 

#[derive(TypeName)]
pub struct MyWorld {
    // this struct contains mutable context
    color: Color,
    c1: Color,
    c2: Color
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a scenario is  started
        MyWorld {
            color: Color {
                ..Default::default()
            },
            c1: Color {
                ..Default::default()
            },
            c2: Color {
                ..Default::default()
            },
        }
    }
}

// fn type_of<T>(_: &T) -> &'static str {
// unsafe { std::intrinsics::type_name::<T>() }
// }

mod colors_steps {
    // Any type that implements cucumber_rust::World + Default can be the world
    use cucumber::steps;
//    use typename::{TypeName};
    use trtc::colors::{Color, color};

    steps!(crate::MyWorld => {
        // Feature: Colors are like red, green, and blue tuples
        given regex r#"c <- color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            world.color = Color{red: r, green: g, blue: b}
        };
        then regex r#"c.red == ([-0-9.]+)"# (f64) | world, r, _step| {
            assert_eq!(world.color.red, r)
        };
        then regex r#"c.green == ([-0-9.]+)"# (f64) | world, g, _step| {
            assert_eq!(world.color.green, g)
        };
        then regex r#"c.blue == ([-0-9.]+)"# (f64) | world, b, _step| {
            assert_eq!(world.color.blue, b)
        };

        // Scenario: Adding colors
        given regex r#"c1 <- color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            world.c1 = color(r, g, b)
        };
        given regex r#"c2 <- color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            world.c2 = color(r, g, b)
        };
        then regex r#"c1 \+ c2 == color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            let added = world.c1 + world.c2;
            assert_eq!(added, color(r, g, b))
        };
                      
        // Scenario: Subtracting colors
        given regex r#"c1 <- color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            world.c1 = color(r, g, b)
        };
        given regex r#"c2 <- color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            world.c2 = color(r, g, b)
        };
        then regex r#"c1 - c2 == color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            let subtracted = world.c1 - world.c2;
            assert_eq!(subtracted, color(r, g, b))
        };

        // Scenario: Multiplying a color by a scalar
        given regex r#"c <- color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            world.color = color(r, g, b)
        };

        then regex r#"c \* ([-0-9.]+) == color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64, f64) | world, multiplier, r, g, b, _step| {
            assert_eq!(world.color * multiplier, color(r, g, b))
        };
        
        // Scenario: Multiplying colors
        given regex r#"c1 <- color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            world.c1 = color(r, g, b)
        };
        given regex r#"c2 <- color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            world.c2 = color(r, g, b)
        };
        then regex r#"c1 \* c2 == color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g, b, _step| {
            let product = world.c1 * world.c2;
            assert_eq!(product, color(r, g, b))
        };

    });
    
}

cucumber! {
    features: "./features/colors", // Path to our feature files
    world: cargo::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        colors_steps::steps // the `steps!` macro creates a `steps` function in a module
    ]
    // setup: setup, // Optional; called once before everything
    // before: &[
    //     a_before_fn // Optional; called before each scenario
    // ],
    // after: &[
    //     an_after_fn // Optional; called after each scenario
    // ls]
}
