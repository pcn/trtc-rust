// Test the implementation of a canvas to draw on
// #![feature(core_intrinsics)]  // for type_of
// extern crate cucumber;
// extern crate typename;
use cucumber::{cucumber};
use typename::{TypeName};

extern crate trtc;
use trtc::canvas::{Canvas, canvas};
use trtc::colors::{Color}; 
use ndarray::Array2;

#[derive(TypeName)]
pub struct MyWorld {
    // this struct contains mutable context
    canvas: Canvas
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a scenario is  started
        MyWorld {
            canvas: canvas(1, 1)
        }
    }
}


mod the_steps {
    // Any type that implements cucumber_rust::World + Default can be the world
    use cucumber::steps;
    use trtc::canvas::{canvas};

    steps!(crate::MyWorld => {
        // Feature: Create a canvas
        given regex r#"c <- canvas\(([-0-9.]+), ([-0-9.]+)\)"# (u32, u32) | world, width, height, _step| {
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            world.canvas = canvas(width, height)
        };
    });
    
}

cucumber! {
    features: "./features/canvas", // Path to our feature files
    world: cargo::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        the_steps::steps // the `steps!` macro creates a `steps` function in a module
    ]
    // setup: setup, // Optional; called once before everything
    // before: &[
    //     a_before_fn // Optional; called before each scenario
    // ],
    // after: &[
    //     an_after_fn // Optional; called after each scenario
    // ls]
}
