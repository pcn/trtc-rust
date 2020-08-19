// #![warn(unused_imports)]
use std::cmp::PartialEq;
use std::ops::Mul; // {Add, Sub};
use crate::equal;

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

#[derive(TypeName, Debug, Clone, Copy)]
pub struct Mat2 {
    pub rows: [[f64; 2]; 2],
}
#[derive(TypeName, Debug, Clone, Copy)]
pub struct Mat3 {
    pub rows: [[f64; 3]; 3],
}
#[derive(TypeName, Debug, Clone, Copy)]
pub struct Mat4 {
    pub rows: [[f64; 4]; 4],
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
            rows: [[0.0, 0.0], [0.0, 0.0]],
        }
    }
}

impl Default for Mat3 {
    fn default() -> Mat3 {
        Mat3 {
            rows: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
        }
    }
}

impl Default for Mat4 {
    fn default() -> Mat4 {
        Mat4 {
            rows: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, other: Mat4) -> Self::Output {
        let mut result = Mat4::default();
        for (r_idx, row) in self.rows.iter().enumerate() {
            for (c_idx, _col) in row.iter().enumerate() {
                result.rows[r_idx][c_idx] = self.rows[r_idx][0] * other.rows[0][c_idx]
                    + self.rows[r_idx][1] * other.rows[1][c_idx]
                    + self.rows[r_idx][2] * other.rows[2][c_idx]
                    + self.rows[r_idx][3] * other.rows[3][c_idx];
            }
        }
        result
    }
}

impl PartialEq for Mat4 {
    fn eq(&self, other: &Mat4) -> bool {
        for (r_idx, row) in self.rows.iter().enumerate() {
            for (c_idx, _) in row.iter().enumerate() {
                if !equal(self.rows[r_idx][c_idx], other.rows[r_idx][c_idx]) {
                    dbg!("r_idx: {:?}, c_idx: {:?} fails with self: {:?} {:?}",
                         r_idx, c_idx, self.rows[r_idx][c_idx], other.rows[r_idx][c_idx]);
                    return false;
                }
            }
        }
        true
    }
}

impl PartialEq for Mat3 {
    fn eq(&self, other: &Mat3) -> bool {
        for (r_idx, row) in self.rows.iter().enumerate() {
            for (c_idx, _) in row.iter().enumerate() {
                if !equal(self.rows[r_idx][c_idx], other.rows[r_idx][c_idx]) {
                    return false;
                }
            }
        }
        true
    }
}

impl PartialEq for Mat2 {
    fn eq(&self, other: &Mat2) -> bool {
        for (r_idx, row) in self.rows.iter().enumerate() {
            for (c_idx, _) in row.iter().enumerate() {
                if !equal(self.rows[r_idx][c_idx], other.rows[r_idx][c_idx]) {
                    return false;
                }
            }
        }
        true
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
    (table.rows[0][0] * table.rows[1][1]) - (table.rows[0][1] * table.rows[1][0])
}

// XXX Check for exclusion sizes being larger than the target size?
pub fn submatrix_4(matrix: Mat4, exclude_row: usize, exclude_col: usize) -> Mat3 {
    let mut result = Mat3::default();
    let mut insert_row: usize = 0;
    let mut insert_col: usize = 0;

    for (r_idx, row) in matrix.rows.iter().enumerate() {
        if r_idx == exclude_row {
            continue;
        };
        for (col_idx, _col) in row.iter().enumerate() {
            if col_idx == exclude_col {
                continue;
            };
            result.rows[insert_row][insert_col] = matrix.rows[r_idx][col_idx];
            insert_col += 1;
        }
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
            continue;
        };
        for (col_idx, _col) in row.iter().enumerate() {
            if col_idx == exclude_col {
                continue;
            };
            result.rows[insert_row][insert_col] = matrix.rows[r_idx][col_idx];
            insert_col += 1;
        }
        insert_col = 0;
        insert_row += 1;
    }
    result
}

pub trait Matrix {
    // From https://doc.rust-lang.org/rust-by-example/trait.html
    fn cofactor(&self, row: usize, col: usize) -> Option<f64>;
    fn determinant(&self) -> f64;
    // Maybe turn this into an option in case the matrix is not invertible,
    // return None?
    fn inverse(&self) -> SmallMatrix;
    // matrix: Mat3, exclude_row: usize, exclude_col: usize) -> Mat2 {
    fn submatrix(&self, exclude_row: usize, exclude_col: usize) -> Option<SmallMatrix>;
    fn minor(&self, exclude_row: usize, exclude_col: usize) -> Option<f64>;
}


impl Matrix for SmallMatrix {
    fn cofactor(&self, row: usize, col: usize) -> Option<f64> {
        let m_m = self.minor
            (row, col);
        if let Some(i) = m_m {
            if (row + col) % 2 == 1 {
                Some(-i)
            } else {
                Some(i)
            }
        } else {
            None
        }
    }

    fn determinant(&self) -> f64 {
        let mut det = 0.0;
        match self {
            SmallMatrix::M2(m) => determinant_2(m.clone()),
            SmallMatrix::M3(m) => {
                for (c_idx, col) in m.rows[0].iter().enumerate() {
                    det += col * self.cofactor(0, c_idx).unwrap();
                }
                det
            },
            SmallMatrix::M4(m) => {
                for (c_idx, col) in m.rows[0].iter().enumerate() {
                    det += col * self.cofactor(0, c_idx).unwrap();
                }
                det
            }
        }
    }

    fn inverse(&self) -> SmallMatrix {
        match self {
            SmallMatrix::M2(m) => {
                let mut new_m = Mat2::default();
                for (r_idx, row) in m.rows.iter().enumerate() {
                    for (c_idx, _) in row.iter().enumerate() {
                        let c = self.cofactor(r_idx, c_idx).unwrap();
                        // Transposition is gotten here by switching the row, col order
                        // in new_m
                        new_m.rows[c_idx][r_idx] = c / self.determinant();
                    }
                }
                SmallMatrix::M2(new_m)
            },
            SmallMatrix::M3(m) => {
                let mut new_m = Mat3::default();
                for (r_idx, row) in m.rows.iter().enumerate() {
                    for (c_idx, _) in row.iter().enumerate() {
                        let c = self.cofactor(r_idx, c_idx).unwrap();
                        // Transposition is gotten here by switching the row, col order
                        // in new_m
                        new_m.rows[c_idx][r_idx] = c / self.determinant();
                    }
                }
                SmallMatrix::M3(new_m)
            }
            SmallMatrix::M4(m) => {
                let mut new_m = Mat4::default();
                for (r_idx, row) in m.rows.iter().enumerate() {
                    for (c_idx, _) in row.iter().enumerate() {
                        let c = self.cofactor(r_idx, c_idx).unwrap();
                        // Transposition is gotten here by switching the row, col order
                        // in new_m
                        new_m.rows[c_idx][r_idx] = c / self.determinant();
                    }
                }
                SmallMatrix::M4(new_m)
            }
        }
    }

    // The minor is the determinant of the submatrix at (i, j). So I'm placing
    // this function near submatrix_3 since the current test relies on a Mat3
    // which probably means something deeper, but until I know more, it is what
    // it is.
    fn minor(&self, exclude_row: usize, exclude_col: usize) -> Option<f64> {
        match self {
            SmallMatrix::M4(m) => {
                let newm = submatrix_4(*m, exclude_row, exclude_col);
                Some(SmallMatrix::M3(newm).determinant())
            }
            SmallMatrix::M3(m) => {
                let newm = submatrix_3(*m, exclude_row, exclude_col);
                Some(SmallMatrix::M2(newm).determinant())
            }
            SmallMatrix::M2(_) => None,
        }
    }

    fn submatrix(&self, exclude_row: usize, exclude_col: usize) -> Option<SmallMatrix> {
        match self {
            SmallMatrix::M4(m) => Some(SmallMatrix::M3(submatrix_4(
                m.clone(),
                exclude_row,
                exclude_col,
            ))),
            SmallMatrix::M3(m) => Some(SmallMatrix::M2(submatrix_3(
                m.clone(),
                exclude_row,
                exclude_col,
            ))),
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
            // dbg!("Col is {:?} {:?}", &col_idx, &col);
            result.rows[r_idx][col_idx] = match col.parse::<f64>() {
                Ok(val) => val,
                Err(_) => {
                    // Boom?
                    panic!("Passed a vec that doesn't fit the target 4x4 array at row {:?} and col {:?}",
                           r_idx, col_idx)
                }
            };
        }
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
            // dbg!("Col is {:?} {:?}", &col_idx, &col);
            result.rows[r_idx][col_idx] = match col.parse::<f64>() {
                Ok(val) => val,
                Err(_) => {
                    // Boom?
                    panic!("Passed a vec that doesn't fit the target 3x3 array at row {:?} and col {:?}",
                           r_idx, col_idx)
                }
            };
        }
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
                Ok(val) => val,
                Err(_) => {
                    // Boom?
                    panic!("Passed a vec that doesn't fit the target 4x4 array at row {:?} and col {:?}",
                           r_idx, col_idx)
                }
            };
        }
        // println!("This is the m4: {:?}", result);
    }
    result
}

// Local Variables:
// cargo-test-arguments: "--test matrix"
// End:
