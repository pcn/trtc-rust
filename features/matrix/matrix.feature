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
  And 1 cofactor(A, 0, 0) == -12
  And minor(A, 1, 0) == 25
  And 1 cofactor(A, 1, 0) == -25

Scenario: Calculating the determinant of a 3x3 matrix
  Given the following 3x3 matrix M3A-5:
    | First | Second | Third |
    |     1 |      2 |     6 |
    |    -5 |      8 |    -4 |
    |     2 |      6 |     4 |
  Then 2 cofactor(A, 0, 0) == 56
  And 2 cofactor(A, 0, 1) == 12
  And 2 cofactor(A, 0, 2) == -46
  And 2 determinant(A) == -196

Scenario: Calculating the determinant of a 4x4 matrix
  Given the following 4x4 matrix M4A-7:
    | First | Second | Third | Fourth |
    |    -2 |     -8 |     3 |      5 |
    |    -3 |      1 |     7 |      3 |
    |     1 |      2 |    -9 |      6 |
    |    -6 |      7 |     7 |     -9 |
  Then 3 cofactor(A, 0, 0) == 690
  And 3 cofactor(A, 0, 1) == 447
  And 3 cofactor(A, 0, 2) == 210
  And 3 cofactor(A, 0, 3) == 51
  And 3 determinant(A) == -4071
 
Scenario: Testing an invertible matrix for invertibility
  Given the following 4x4 matrix M4A-8:
    | First | Second | Third | Fourth |
    |     6 |      4 |     4 |      4 |
    |     5 |      5 |     7 |      6 |
    |     4 |     -9 |     3 |     -7 |
    |     9 |      1 |     7 |     -6 |
  Then determinant(M4A-8) == -2120
  And M4A-8 is invertible

Scenario: Testing a noninvertible matrix for invertibility
  Given the following 4x4 matrix M4A-9:
    | First | Second | Third | Fourth |
    |    -4 |      2 |    -2 |     .3 |
    |     9 |      6 |     2 |      6 |
    |     0 |     -5 |     1 |     -5 |
    |     0 |      0 |     0 |      0 |
  Then determinant(M4A-9) == 0
  And M4A-9 is not invertible

Scenario: Calculating the inverse of a matrix
  Given the following 4x4 matrix M4A-10:
    | First | Second | Third | Fourth |
    |    -5 |      2 |     6 |     -8 |
    |     1 |     -5 |     1 |      8 |
    |     7 |      7 |    -6 |     -7 |
    |     1 |     -3 |     7 |      4 |
  And B <- inverse(M4A-10)
  Then determinant(M4A-10) == 532
  And cofactor(M4A-10, 2, 3) == -160
  And B[3,2] == -160/532
  And B is the following 4x4 matrix:
    |    First |   Second |    Third |   Fourth |
    |  0.21805 |  0.45113 |   .24060 | -0.04511 |
    | -0.80827 | -1.45677 | -0.44361 |  0.52068 |
    | -0.07895 | -0.22368 | -0.05263 |  0.19737 |
    | -0.52256 | -0.81391 | -0.30075 |  0.30639 |
    
Scenario: Calculating the inverse of another matrix
  Given the following 4x4 matrix M4A-11:
    | First | Second | Third | Fourth |
    |     8 |     -5 |     9 |      2 |
    |     7 |      5 |     6 |      1 |
    |    -6 |      0 |     9 |      6 |
    |    -3 |      0 |    -9 |     -4 |
  Then inverse(M4A-11) is the following 4x4 matrix:
    |    First |   Second |    Third |   Fourth |
    | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
    | -0.07692 |  0.12308 |  0.02564 |  0.03077 |
    |  0.35897 |  0.35897 |  0.43590 |  0.92308 |
    | -0.69231 | -0.69231 | -0.76923 | -1.92308 |
    
Scenario: Calculating the inverse of a third matrix
  Given the following 4x4 matrix M4A-12:
    | First | Second | Third | Fourth |
    |     9 |      3 |     0 |      9 |
    |    -5 |     -2 |    -6 |     -3 |
    |    -4 |      9 |     6 |      4 |
    |    -7 |      6 |     6 |      2 |
  Then inverse(M4A-12) is the following 4x4 matrix:
    |    First |   Second |    Third |   Fourth |
    | -0.04074 | -0.07778 |  0.14444 | -0.22222 |
    | -0.07778 |  0.03333 |  0.36667 | -0.33333 |
    | -0.02901 | -0.14630 | -0.10926 |  0.12963 |
    |  0.17778 |  0.06667 | -0.26667 |  0.33333 |

Scenario: Multiplying a product by its inverse
  Given the following 4x4 matrix M4A-13:
    | First | Second | Third | Fourth |
    |     3 |     -9 |     7 |      3 |
    |     3 |     -8 |     2 |     -9 |
    |    -4 |      4 |     4 |      1 |
    |    -6 |      5 |    -1 |      1 |
  And the following 4x4 matrix M4B-13:
    | First | Second | Third | Fourth |
    |     8 |      2 |     2 |      2 |
    |     3 |     -1 |     7 |      0 |
    |     7 |      0 |     5 |      4 |
    |     6 |     -2 |     0 |      5 |
  And C <- M4A-13 * M4B-13
  Then C * inverse(M4B-13) == A
 



    


