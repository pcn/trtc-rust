// #![warn(unused_imports)]
use std::ops::Mul; // {Add, Sub};
use std::cmp::PartialEq;

// TODO: implement a from vec for this, with something like this:
//
//     for (rcount, row) in rows.into_iter().enumerate() {
//         for (ccount, col) in row.into_iter().enumerate() {
//             println!("Col is {:?} {:?}", ccount, col);
//             m4.rows[rcount][ccount] = col.parse::<f64>().unwrap();
//         }
//         println!("This is the m4: {:?}", m4)
//     }
// 
// matrix 2x2 to 4x4

#[derive(TypeName, Debug, Clone, Copy, PartialEq)]
pub struct Mat2 {
    pub rows: [[f64; 2]; 2]
}
#[derive(TypeName, Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub rows: [[f64; 3]; 3]
}
#[derive(TypeName, Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub rows: [[f64; 4]; 4]
}

#[derive(TypeName, Debug, Clone, Copy, PartialEq)]
pub enum SmallMatrix {
    M2(Mat2),
    M3(Mat3),
    M4(Mat4),
}
    

impl Default for Mat2 {
    fn default() -> Mat2 {
        Mat2 {
            rows: [[0.0, 0.0], [0.0, 0.0]]
        }
    }
}


impl Default for Mat3 {
    fn default() -> Mat3 {
        Mat3 {
            rows: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]
        }
    }
}

impl Default for Mat4 {
    fn default() -> Mat4 {
        Mat4 {
            rows: [[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]]
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, other: Mat4) -> Self::Output {
        let mut result = Mat4::default();
        for (r_idx, row) in self.rows.iter().enumerate() {
            for (c_idx, _col) in row.iter().enumerate() {
                result.rows[r_idx][c_idx] =
                    self.rows[r_idx][0] * other.rows[0][c_idx] +
                    self.rows[r_idx][1] * other.rows[1][c_idx] +
                    self.rows[r_idx][2] * other.rows[2][c_idx] +
                    self.rows[r_idx][3] * other.rows[3][c_idx];
            }
        }
        result
    }
}

pub fn transpose_m4(matrix: Mat4) -> Mat4 {
    let mut result = Mat4::default();
    for (r_idx, row) in matrix.rows.iter().enumerate() {
        for (c_idx, _) in row.iter().enumerate() {
            result.rows[c_idx][r_idx] = matrix.rows[r_idx][c_idx]
        }
    }
    result
}


pub fn determinant_2(table: Mat2) -> f64 {
    (table.rows[0][0] * table.rows[1][1]) -
        (table.rows[0][1] * table.rows[1][0])
}

// XXX Check for exclusion sizes being larger than the target size?
pub fn submatrix_4(matrix: Mat4, exclude_row: usize, exclude_col: usize) -> Mat3 {
    let mut result = Mat3::default();
    let mut insert_row: usize = 0;
    let mut insert_col: usize = 0;
        
    for (r_idx, row) in matrix.rows.iter().enumerate() {
        if r_idx == exclude_row {
            continue
        };
        for (col_idx, _col) in row.iter().enumerate() {
            if col_idx == exclude_col {
                continue
            };
            result.rows[insert_row][insert_col] = matrix.rows[r_idx][col_idx];
            insert_col += 1;
        };
        insert_col = 0;
        insert_row += 1;
    }
    result
}

// XXX Check for exclusion sizes being larger than the target size?
pub fn submatrix_3(matrix: Mat3, exclude_row: usize, exclude_col: usize) -> Mat2 {
    let mut result = Mat2::default();
    let mut insert_row: usize = 0;
    let mut insert_col: usize = 0;
        
    for (r_idx, row) in matrix.rows.iter().enumerate() {
        if r_idx == exclude_row {
            continue
        };
        for (col_idx, _col) in row.iter().enumerate() {
            if col_idx == exclude_col {
                continue
            };
            result.rows[insert_row][insert_col] = matrix.rows[r_idx][col_idx];
            insert_col += 1;
        };
        insert_col = 0;
        insert_row += 1;
    }
    result
}

// The minor is the determinant of the submatrix at (i, j). So I'm placing
// this function near submatrix_3 since the current test relies on a Mat3
// which probably means something deeper, but until I know more, it is what
// it is.
pub fn minor(matrix: Mat3, exclude_row: usize, exclude_col: usize) -> f64 {
    let m2 = submatrix_3(matrix, exclude_row, exclude_col);
    determinant_2(m2)
}

trait Matrix {
    // From https://doc.rust-lang.org/rust-by-example/trait.html
    fn cofactor(matrix: &Self, row: usize, col: usize) -> Option<f64>;
    // matrix: Mat3, exclude_row: usize, exclude_col: usize) -> Mat2 {    
    fn submatrix(matrix: &Self, exclude_row: usize, exclude_col: usize) -> Option<SmallMatrix>;
}

impl Matrix for SmallMatrix {
    fn cofactor(matrix: &Self, row: usize, col: usize) -> Option<f64> {
        // The cofactor is only for the 3x3 matrix
        match matrix {
            SmallMatrix::M3(m) => {
                let m_m = minor(m.clone(), row, col);
                if (row + col) % 2 == 1 {
                    Some(-m_m)
                } else {
                    Some(m_m)
                }
            },
            _ => None
        }
    }

    fn submatrix(matrix: &Self, exclude_row: usize, exclude_col: usize) -> Option<SmallMatrix> {
        match matrix {
            SmallMatrix::M4(m) => Some(SmallMatrix::M3(submatrix_4(m.clone(), exclude_row, exclude_col))),
            SmallMatrix::M3(m) => Some(SmallMatrix::M2(submatrix_3(m.clone(), exclude_row, exclude_col))),
            SmallMatrix::M2(_) => None,
        }
    }
}
        

    


// This is for use with the gherkin tests' table type - I think I'm going to want to avoid
// doing conversions from vec to Mat4 in general, to avoid possible runtime
// crashes, but for tests, I'll have a from_vecN() function for each to get me moving again.
// TODO: Maybe just take a table type from cucumber?
pub fn from_vec4(table_rows: Vec<Vec<String>>) -> Mat4 {
    let mut result = Mat4::default();
    for (r_idx, row) in table_rows.into_iter().enumerate() {
        for (col_idx, col) in row.into_iter().enumerate() {
            dbg!("Col is {:?} {:?}", &col_idx, &col);
            result.rows[r_idx][col_idx] = match col.parse::<f64>() {
                Ok(val) => {
                    val
                },
                Err(_) => {
                    // Boom?
                    panic!("Passed a vec that doesn't fit the target 4x4 array at row {:?} and col {:?}",
                           r_idx, col_idx)
                }
            };
        };
        // println!("This is the m4: {:?}", result);
    }
    result
}



// This is for use with the gherkin tests' table type - I think I'm going to want to avoid
// doing conversions from vec to Mat4 in general, to avoid possible runtime
// crashes, but for tests, I'll have a from_vecN() function for each to get me moving again.
pub fn from_vec3(table_rows: Vec<Vec<String>>) -> Mat3 {
    let mut result = Mat3::default();
    for (r_idx, row) in table_rows.into_iter().enumerate() {
        for (col_idx, col) in row.into_iter().enumerate() {
            dbg!("Col is {:?} {:?}", &col_idx, &col);
            result.rows[r_idx][col_idx] = match col.parse::<f64>() {
                Ok(val) => {
                    val
                },
                Err(_) => {
                    // Boom?
                    panic!("Passed a vec that doesn't fit the target 3x3 array at row {:?} and col {:?}",
                           r_idx, col_idx)
                }
            };
        };
        // println!("This is the m3: {:?}", result);
    }
    result
}

// This is for use with the gherkin tests' table type - I think I'm going to want to avoid
// doing conversions from vec to Mat4 in general, to avoid possible runtime
// crashes, but for tests, I'll have a from_vecN() function for each to get me moving again.
pub fn from_vec2(table_rows: Vec<Vec<String>>) -> Mat2 {
    let mut result = Mat2::default();
    for (r_idx, row) in table_rows.into_iter().enumerate() {
        for (col_idx, col) in row.into_iter().enumerate() {
            dbg!("Col is {:?} {:?}", &col_idx, &col);
            result.rows[r_idx][col_idx] = match col.parse::<f64>() {
                Ok(val) => {
                    val
                },
                Err(_) => {
                    // Boom?
                    panic!("Passed a vec that doesn't fit the target 4x4 array at row {:?} and col {:?}",
                           r_idx, col_idx)
                }
            };
        };
        // println!("This is the m4: {:?}", result);
    }
    result
}
