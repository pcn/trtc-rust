// #![feature(core_intrinsics)]  // for type_of
extern crate cucumber;
extern crate typename;
use cucumber::{after, before, cucumber, steps};
use typename::TypeName;

extern crate trtc;
use trtc::colors::{Color,color};

#[derive(TypeName)]
pub struct MyWorld {
    // this struct contains mutable context
    color: Color,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a scenario is  started
        MyWorld {
            color: Color {
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
    use typename::TypeName;
    use trtc::colors::{Color, color};


    steps!(::MyWorld => {
        // Feature: Colors are like red, green, and blue tuples
        given regex r#"c <- color\(([-0-9.]+), ([-0-9.]+), ([-0-9.]+)\)"# (f64, f64, f64) | world, r, g , b, _step| {
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

    });
    
}

cucumber! {
    features: "./features", // Path to our feature files
    world: ::MyWorld, // The world needs to be the same for steps and the main cucumber call
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
