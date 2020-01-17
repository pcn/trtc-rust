extern crate trtc;
extern crate docopt;
extern crate serde;

use docopt::Docopt;
use serde::Deserialize;

use trtc::vectors::{Vector, vector, normalize};
use trtc::points::{Point, point};

const USAGE: &'static str = "
Watch a projectile position as it ticks

Usage:
  ch1-projectile-tick [--gravity=<g>] [--wind=<w>] [--sx=<sx>] [--sy=<sy>] [--sz=<sz>] [--vx=<vx>] [--vy=<vy>] [--vz=<vz>]
  ch1-projectile-tick (-h | --help)
  ch1-projectile-tick --version

Options:
  --gravity=<g>    gravity in the y direction [default: -0.1].
  --wind=<w>       wind force in the x direction [default: -0.01].
  --sx=<sx>   start position x [default: 0.0].
  --sy=<sw>   start position y [default: 1.0].
  --sz=<sz>   start position z [default: 0.0].
  --vx=<vx>       wind force in the x direction [default: 1.0].
  --vy=<vy>       wind force in the x direction [default: 1.0].
  --vz=<vz>       wind force in the x direction [default: 0.0].
  -h --help        Show this screen.
  --version        Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_gravity: f64,
    flag_wind: f64,
    flag_sx: f64,
    flag_sy: f64,
    flag_sz: f64,
    flag_vx: f64,
    flag_vy: f64,
    flag_vz: f64
}

#[derive(Debug, PartialEq)]
struct Projectile {
    position: Point,
    velocity: Vector
}
#[derive(Debug, PartialEq)]
struct Environment {
    gravity: Vector,
    wind: Vector
}

// Returns a new Projectile struct
// This doesn't allow for a negative result to happen

fn projectile_tick(env: &Environment, proj: &Projectile) -> Projectile {
    Projectile {
            position: proj.position + proj.velocity,
            velocity: proj.velocity + env.gravity + env.wind
    }
}

fn tick(env: &Environment, proj: &Projectile) -> Option<Projectile> {
    if (proj.position + proj.velocity).y < 0.0 {
        None
    } else {
        Some(projectile_tick(env, proj))
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.options_first(true).deserialize())
        .unwrap_or_else(|e| e.exit());    
    println!("{:?}", args);    
    let mut p = Projectile{
        position: point(args.flag_sx, args.flag_sy, args.flag_sz),
        velocity: normalize(&vector(args.flag_vx, args.flag_vy, args.flag_vz))
    };
    let env = Environment {
        gravity: vector(0.0, args.flag_gravity, 0.0),
        wind: vector(args.flag_wind, 0.0, 0.0)
    };
    while let Some(newp) = tick(&env, &p) {
        println!("Projectile is at {:?}", newp);
        p = newp;
    }
    println!("Projectile ended up at {:?}", projectile_tick(&env, &p));
}
         
