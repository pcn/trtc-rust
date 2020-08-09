Feature: Matrices
Scenario: Constructing and inspecting a 4x4 matrix
  Given the following 4x4 matrix M4:
  | First | Second | Third | Fourth |
  |   1.0 |    2.0 |   3.0 |    4.0 |
  |   5.5 |    6.5 |   7.5 |    8.5 |
  |     9 |     10 |    11 |     12 |
  |  13.5 |   14.5 |  15.5 |   16.5 |
  Then M4[0,0] == 1.0
  And M4[0,3] == 4.0
  And M4[1,0] == 5.5
  And M4[1,2] == 7.5
  And M4[2,2] == 11.0
  And M4[3,0] == 13.5
  And M4[3,2] == 15.5

Scenario: A 2x2 matrix ought to be representable
  Given the following 2x2 matrix M2:
  | first | second |
  |    -3 |      5 |
  |     1 |     -2 |
  Then M2[0,0] == -3.0
  And M2[0,1] == 5.0
  And M2[1,0] == 1.0
  And M2[1,1] == -2.0

Scenario: A 3x3 matrix ought to be representable
  Given the following 3x3 matrix M3:
    | first | second | third |
    |    -3 |      5 |     0 |
    |     1 |     -2 |    -7 |
    |     0 |      1 |     1 |
  Then M3[0,0] == -3.0
  And M3[1,1] == -2.0
  And M3[2,2] == 1.0

Scenario: Matrix equality with identical matrices
  Given the following matrix M4A:
    | first | second | third | fourth |
    |     1 |      2 |     3 |      4 |
    |     5 |      6 |     7 |      8 |
    |     9 |      8 |     7 |      6 |
    |     5 |      4 |     3 |      2 |
  And the following matrix M4B:
    | first | second | third | fourth |
    |     1 |      2 |     3 |      4 |
    |     5 |      6 |     7 |      8 |
    |     9 |      8 |     7 |      6 |
    |     5 |      4 |     3 |      2 |
    Then M4A == M4B

Scenario: Matrix equality with different matrices
  Given the following matrix M4A-2:
    | first | second | third | fourth |
    |     1 |      2 |     3 |      4 |
    |     5 |      6 |     7 |      8 |
    |     9 |      8 |     7 |      6 |
    |     5 |      4 |     3 |      2 |
  And the following matrix M4B-2:
    | first | second | third | fourth |
    |     2 |      3 |     4 |      5 |
    |     6 |      7 |     8 |      9 |
    |     8 |      7 |     6 |      5 |
    |     4 |      3 |     2 |      1 |
  Then M4A-2 != M4B-2

Scenario: Multiplying two matrices
  Given the following matrix M4A-3:
    | first | second | third | fourth |
    |     1 |      2 |     3 |      4 |
    |     5 |      6 |     7 |      8 |
    |     9 |      8 |     7 |      6 |
    |     5 |      4 |     3 |      2 |
  And the following matrix M4B-3:
    | first | second | third | fourth |
    |    -2 |      1 |     2 |      3 |
    |     3 |      2 |     1 |     -1 |
    |     4 |      3 |     6 |      5 |
    |     1 |      2 |     7 |      8 |
  Then M4A-3 * M4B-3 is the following 4x4 matrix:
    | first | second | third | fourth |
    |    20 |     22 |    50 |     48 |
    |    44 |     54 |   114 |    108 |
    |    40 |     58 |   110 |    102 |
    |    16 |     26 |    46 |     42 |


Scenario: Multiplying a matrix by the identity matrix
  Given the following matrix M4A-4:
    | first | second | third | fourth |
    |     1 |      2 |     3 |      4 |
    |     5 |      6 |     7 |      8 |
    |     9 |      8 |     7 |      6 |
    |     5 |      4 |     3 |      2 |
  Then M4A-4 * identity_matrix == M4A-4

Scenario: Transposing a matrix
  Given the following matrix M4A-5:
    | first | second | third | fourth |
    |     0 |      9 |     3 |      0 |
    |     9 |      8 |     0 |      8 |
    |     1 |      8 |     5 |      3 |
    |     0 |      0 |     5 |      8 |
  Then transpose(M4A-5) is the following matrix: 
    | first | second | third | fourth |
    |     0 |      9 |     1 |      0 |
    |     9 |      8 |     8 |      0 |
    |     3 |      0 |     5 |      5 |
    |     0 |      8 |     3 |      8 |

Scenario: Calculating the determinant of a 2x2 matrix
  Given the following 2x2 matrix M2A:
    | first | second |
    |     1 |      5 |
    |    -3 |      2 |
  Then determinant_2(M2A) == 17

Scenario: A submatrix of a 3x3 matrix is a 2x2 matrix
  Given the following 3x3 matrix M3A-2:
    | first | second | third |
    |     1 |      5 |     0 |
    |    -3 |      2 |     7 |
    |     0 |      6 |    -3 |
  Then submatrix_3(M3A-2, 0, 2) is the following 2x2 matrix:
    | first | second |
    |    -3 |      2 |
    |     0 |      6 |

Scenario: A submatrix of a 4x4 matrix is a 3x3 matrix
  Given the following 4x4 matrix M4A-6
    | first | second | third | fourth |
    |    -6 |      1 |     1 |      6 |
    |    -8 |      5 |     8 |      6 |
    |    -1 |      0 |     8 |      2 |
    |    -7 |      1 |    -1 |      1 |
  Then submatrix_4(M4A-6, 2, 1) is the following 3x3 matrix:
    | first | second | third |
    |    -6 |      1 |     6 |
    |    -8 |      8 |     6 |
    |    -7 |     -1 |     1 |

Scenario: Calculating a minor of a 3x3 matrix
  Given the following 3x3 matrix M3A-3:
    | first | second | third |
    |     3 |      5 |     0 |
    |     2 |     -1 |    -7 |
    |     6 |     -1 |     5 |
  And B <- submatrix(M3A-3, 1, 0)
  Then determinant(B) == 25
  And minor(M3A-3, 1, 0) == 25

Scenario: Calculating a cofactor of a 3x3 matrix
  Given the following 3x3 matrix M3A-4:
    | First | Second | Third |
    |     3 |      5 |     0 |
    |     2 |     -1 |    -7 |
    |     6 |     -1 |     5 |
  Then minor(A, 0, 0) == -12
  And cofactor(A, 0, 0) == -12
  And minor(A, 1, 0) == 25
  And cofactor(A, 1, 0) == -25

Scenario: Calculating the determinant of a 3x3 matrix
  Given the following 3x3 matrix M3A-5:
    | First | Second | Third |
    |     1 |      2 |     6 |
    |    -5 |      8 |    -4 |
    |     2 |      6 |     4 |
  Then cofactor(A, 0, 0) == 690
  And cofactor(A, 0, 1) == 447
  And cofactor(A, 0, 2) == 210
  And cofactor(A, 0, 3) == 51
  And determinant(A) == -4071

Scenario: Calculating the determinant of a 4x4 matrix
  Given the following 4x4 matrix M4A-7:
    | First | Second | Third | Fourth |
    |    -2 |     -8 |     3 |      5 |
    |    -3 |      1 |     7 |      3 |
    |     1 |      2 |    -9 |      6 |
    |    -6 |      7 |     7 |     -9 |
  Then cofactor(A, 0, 0) == 690
  And cofactor(A, 0, 1) == 447
  And cofactor(A, 0, 2) == 210
  And cofactor(A, 0, 3) == 51
  And determinant(A) == -4071
 
