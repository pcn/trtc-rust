// #![feature(core_intrinsics)]  // for type_of
extern crate cucumber;
extern crate typename;

use cucumber::{after, before, cucumber, steps};
use typename::TypeName;

extern crate trtc;
use trtc::matrix::{Mat2,Mat3,Mat4};


#[derive(TypeName)]
pub struct MyWorld {
    // this struct contains mutable context
    m2: Mat2,
    m3: Mat3,
    m4: Mat4,
    m4b: Mat4,
//    sm1: SmallMatrix,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a scenario is  started
        MyWorld {
            m2: Mat2 {
                ..Default::default()
            },
            m3: Mat3 {
                ..Default::default()
            },
            m4: Mat4 {
                ..Default::default()
            },
            m4b: Mat4 {
                ..Default::default()
            },
//            sm1: 
        }
    }
}


mod tuples_steps {
    // Any type that implements cucumber_rust::World + Default can be the world
    use cucumber::steps;
    use typename::TypeName;
    use trtc::matrix::{Mat2, Mat3, Mat4, SmallMatrix, from_vec2, from_vec3, from_vec4,
                       transpose_m4, determinant_2, submatrix_3, submatrix_4, minor};
    steps!(crate::MyWorld => {
        // Scenario: Constructing and inspecting a 4x4 matrix
        given r#"the following 4x4 matrix M4:"#  | world, _step| {
            // XXX: cucumber-rust wants the first line of the table to be a heading,
            // which the book seems to not do.  So I'm going to change the feature file.
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            let table = _step.table().unwrap().clone();
            world.m4 = from_vec4(table.rows);
        };
        then "M4[0,0] == 1.0" |world, _step| {
            dbg!(world.m4);
            assert_eq!(world.m4.rows[0][0], 1.0);
        };
        then "M4[0,3] == 4.0" | world, _step| {
            assert_eq!(world.m4.rows[0][3], 4.0);
        };
        then "M4[1,0] == 5.5" | world, _step| {
            assert_eq!(world.m4.rows[1][0], 5.5);
        };
        then "M4[1,2] == 7.5" | world, _step| {
            assert_eq!(world.m4.rows[1][2], 7.5);
        };
        then "M4[2,2] == 11.0" | world, _step| {
            assert_eq!(world.m4.rows[2][2], 11.0);
        };
        then "M4[3,0] == 13.5" | world, _step| {
            assert_eq!(world.m4.rows[3][0], 13.5);
        };
        then "M4[3,2] == 15.5" | world, _step| {
            assert_eq!(world.m4.rows[3][2], 15.5);
        };
        given "the following 2x2 matrix M2:"  | world, _step| {
            // XXX: cucumber-rust wants the first line of the table to be a heading,
            // which the book seems to not do.  So I'm going to change the feature file.
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            let table = _step.table().unwrap().clone();
            world.m2 = from_vec2(table.rows);
        };
        then "M2[0,0] == -3.0" |world, _step| {
            assert_eq!(world.m2.rows[0][0], -3.0);
        };
        then "M2[0,1] == 5.0" |world, _step| {
            assert_eq!(world.m2.rows[0][1], 5.0);
        };
        then "M2[1,0] == 1.0" |world, _step| {
            assert_eq!(world.m2.rows[1][0], 1.0);
        };
        then "M2[1,1] == -2.0" |world, _step| {
            assert_eq!(world.m2.rows[1][1], -2.0);
        };
        given "the following 3x3 matrix M3:"  | world, _step| {
            // XXX: cucumber-rust wants the first line of the table to be a heading,
            // which the book seems to not do.  So I'm going to change the feature file.
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            let table = _step.table().unwrap().clone();
            world.m3 = from_vec3(table.rows);
        };
        then "M3[0,0] == -3.0" |world, _step| {
            assert_eq!(world.m3.rows[0][0], -3.0);
        };
        then "M3[1,1] == -2.0" |world, _step| {
            assert_eq!(world.m3.rows[1][1], -2.0);
        };
        then "M3[2,2] == 1.0" |world, _step| {
            assert_eq!(world.m3.rows[2][2], 1.0);
        };
        given "the following matrix M4A:"  | world, _step| {
            // XXX: cucumber-rust wants the first line of the table to be a heading,
            // which the book seems to not do.  So I'm going to change the feature file.
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            let table = _step.table().unwrap().clone();
            world.m4 = from_vec4(table.rows);
        };
        given "the following matrix M4B:" | world, _step| {
            let table = _step.table().unwrap().clone();
            world.m4b = from_vec4(table.rows);
        };
        then "M4A == M4B" | world, _step| {
            assert_eq!(world.m4, world.m4b)
        };
        given "the following matrix M4A-2:"  | world, _step| {
            // XXX: cucumber-rust wants the first line of the table to be a heading,
            // which the book seems to not do.  So I'm going to change the feature file.
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            let table = _step.table().unwrap().clone();
            world.m4 = from_vec4(table.rows);
        };
        given "the following matrix M4B-2:" | world, _step| {
            let table = _step.table().unwrap().clone();
            world.m4b = from_vec4(table.rows);
        };
        then "M4A-2 != M4B-2" | world, _step| {
            assert_ne!(world.m4, world.m4b)
        };
        given "the following matrix M4A-3:"  | world, _step| {
            // XXX: cucumber-rust wants the first line of the table to be a heading,
            // which the book seems to not do.  So I'm going to change the feature file.
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            let table = _step.table().unwrap().clone();
            world.m4 = from_vec4(table.rows);
        };
        given "the following matrix M4B-3:" | world, _step| {
            let table = _step.table().unwrap().clone();
            world.m4b = from_vec4(table.rows);
        };
        then "M4A-3 * M4B-3 is the following 4x4 matrix:" | world, _step| {
            let table = _step.table().unwrap().clone();
            let expected_result = from_vec4(table.rows);
            let actual_result = world.m4 * world.m4b;
            assert_eq!(expected_result, actual_result);
        };
        given "the following matrix M4A-4:"  | world, _step| {
            // XXX: cucumber-rust wants the first line of the table to be a heading,
            // which the book seems to not do.  So I'm going to change the feature file.
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            let table = _step.table().unwrap().clone();
            world.m4 = from_vec4(table.rows);
        };
        then "M4A-4 * identity_matrix == M4A-4" |world, _step| {
            let identity_matrix = Mat4{ rows: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] };
            let result = world.m4 * identity_matrix;
            assert_eq!(result, world.m4)
        };
        given "the following matrix M4A-5:"  | world, _step| {
            // XXX: cucumber-rust wants the first line of the table to be a heading,
            // which the book seems to not do.  So I'm going to change the feature file.
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            let table = _step.table().unwrap().clone();
            world.m4 = from_vec4(table.rows);
        };
        then "transpose(M4A-5) is the following matrix:" |world, _step| {
            let table = _step.table().unwrap().clone();
            let expected_result = from_vec4(table.rows);
            let actual_result = transpose_m4(world.m4);
            assert_eq!(expected_result, actual_result);
        };
        given "the following 2x2 matrix M2A:"  | world, _step| {
            // XXX: cucumber-rust wants the first line of the table to be a heading,
            // which the book seems to not do.  So I'm going to change the feature file.
            // XXX I don't think this is a good thing for types. Re-visit this later? -PCN
            let table = _step.table().unwrap().clone();
            world.m2 = from_vec2(table.rows);
        };
        then "determinant_2(M2A) == 17" |world, _step| {
            assert_eq!(determinant_2(world.m2), 17.0)
        };
        given "the following 3x3 matrix M3A-2:" |world, _step| {
            let table = _step.table().unwrap().clone();
            world.m3 = from_vec3(table.rows);
        };
        then "submatrix_3(M3A-2, 0, 2) is the following 2x2 matrix:" |world, _step| {
            let table = _step.table().unwrap().clone();
            let expected_result = from_vec2(table.rows);
            let actual_result = submatrix_3(world.m3, 0, 2);
            assert_eq!(expected_result, actual_result);
        };            
        
        given "the following 4x4 matrix M4A-6" |world, _step| {
            let table = _step.table().unwrap().clone();
            world.m4 = from_vec4(table.rows);
        };
        then "submatrix_4(M4A-6, 2, 1) is the following 3x3 matrix:" |world, _step| {
            let table = _step.table().unwrap().clone();
            let expected_result = from_vec3(table.rows);
            let actual_result = submatrix_4(world.m4, 2, 1);
            assert_eq!(expected_result, actual_result);
        };

        // Scenario: Calculating a minor of a 3x3 matrix
        given "the following 3x3 matrix M3A-3:"  |world, _step| {
            let table = _step.table().unwrap().clone();
            world.m3 = from_vec3(table.rows);
        };
        given regex r#"B <- submatrix\(M3A-3, ([-0-9]+), ([-0-9]+)\)"# (usize, usize) | world, exclude_row, exclude_col, _step | {
            world.m2 = submatrix_3(world.m3, exclude_row, exclude_col);
        };
        then regex r#"determinant\(B\) == ([-0-9.]+)"# (f64) | world, result, _step | {
            assert_eq!(determinant_2(world.m2), result);
        };
        then regex r#"minor\(M3A-3, ([-0-9.]+), ([-0-9.]+)\) == ([-0-9.]+)"# (usize, usize, f64) | world, exclude_row, exclude_col, result, _step | {
            assert_eq!(minor(world.m3, exclude_row, exclude_col), result);
        };
       // Scenario: Calculating a cofactor of a 3x3 matrix
        given "the following 3x3 matrix M3A-4:"  |world, _step| {
            let table = _step.table().unwrap().clone();
            world.m3 = from_vec3(table.rows);
        };
        then regex r#"minor\(A, ([-0-9.]+), ([-0-9.]+)\) == ([-0-9.]+)"# (usize, usize, f64) | world, exclude_row, exclude_col, result, _step | {
            assert_eq!(minor(world.m3, exclude_row, exclude_col), result)
        };
        then regex r#"cofactor\(A, ([-0-9.]+), ([-0-9.]+)\) == ([-0-9.]+)"# (usize, usize, f64) | world, row, col, result, _step | {
            let mat = SmallMatrix::M3(world.m3);
            println!("{:?}", mat);
            assert_eq!(mat.cofactor(row, col).unwrap(), result)
        };
         
    });
    
}

cucumber! {
    features: "./features/matrix", // Path to our feature files
    world: crate::MyWorld, // The world needs to be the same for steps and the main cucumber call
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
