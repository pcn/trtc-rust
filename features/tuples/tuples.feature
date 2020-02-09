Feature: A tuple 2 different ways gives us either a point or a vector
  Scenario: A tuple with w=1 is a point
    Given a is a tuple of 4.3, -4.2, 3.1, 1
    Then a.x = 4.3
     And a.y = -4.2
     And a.z = 3.1
     And a.w = 1
     And a is a point
     And a is not a vector

  Scenario: A tuple with w=0 is a vector
    Given b is a tuple of 4.3, -4.2, 3.1, 0
    Then b.x = 4.3
     And b.y = -4.2
     And b.z = 3.1
     And b.w = 0
     And b is a vector
     And b is not a point

  # #  I don't think these two scenarios make sense with
  # #  what I'm doing in rustlang
  # Scenario: Point describes a tuple with type trtc::tuples::Point and w=1
  #   Given p = point(4.0, -4.0, 3)
  #   Then p == tuple(4, -4, 3, 1)

  # Scenario: "vector" describes tuples with w=0
  #   Given v = vector (4, -4, 3)
  #   Then v == tuple(4, -4, 3, 0)


  Scenario: Adding two tuples - one point and one vector
    Given a1 <- point(3, -2, 5)
    And a2 <- vector(-2, 3, 1)
    Then a1 + a2 == point(1, 1, 6)

  Scenario: Subtracting two points
    Given p1 <- point(3, 2, 1)
    And p2 <- point(5, 6, 7)
    Then p1 - p2 == vector(-2, -4, -6)

  Scenario: Subtacting a vector from a point
    Given p <- point(3, 2, 1)
    And v <- vector(5, 6, 7)
    Then p - v == point(-2, -4, -6) 

  Scenario: Subtacting two vectors
    Given v1 <- vector(3, 2, 1)
    And v2 <- vector(5, 6, 7)
    Then v1 - v2 == vector(-2, -4, -6) 

  Scenario: Subtracting a vector from the zero vector
    Given zero <- vector(0, 0, 0)
    And v_sub <- vector(1, -2, 3)
    Then zero - v == vector(-1, 2, -3)

  Scenario: Negating a tuple
    Given c <- tuple(1, -2, 3, -4)
    Then -c == tuple(-1, 2, -3, 4)

  Scenario: Negating a vector
    Given bneg <- vector(1, -2, 3)
    Then -bneg == vector(-1, 2, -3)

  Scenario: Multiplying a tuple by a scalar
    Given c <- tuple(1, -2, 3, -4)
    Then c * 3.5 == tuple(3.5, -7, 10.5, -14)

  Scenario: Multiplying a tuple by a fraction
    Given c_frac <- tuple(1, -2, 3, -4)
    Then c_frac * 0.5 == tuple(0.5, -1, 1.5, -2)

  Scenario: Computing the magnitude of vector(1, 0, 0)
    Given v100 <- vector(1, 0, 0)
    Then magnitude(v100) == 1 

  Scenario: Computing the magnitude of vector(0, 1, 0)
    Given v010 <- vector(0, 1, 0)
    Then magnitude(v010) == 1

  Scenario: Computing the magnitude of vector(0, 0, 1)
    Given v001 <- vector(0, 0, 1)
    Then magnitude(v001) == 1

  Scenario: Computing the magnitude of vector(1, 2, 3)
    Given v123 <- vector(1, 2, 3)
    Then magnitude(v123) == sqrt(14)

  Scenario: Computing the magnitude of vector(-1, -2, -3)
    Given vneg123 <- vector(1, 2, 3)
    Then magnitude(vneg123) == sqrt(14)

  Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
    Given v <- vector(4, 0, 0)
    Then normalize(vnorm) == vector(1, 0, 0)

  Scenario: Normalizing vector(1, 2, 3)
    Given vnorm123 <- vector(1, 2, 3)
    # Approximately 1/sqrt(14), 2/sqrt(14), 3/sqrt(14)
    Then normalize(vnorm123) == approximately vector(0.26726, 0.53452, 0.80178)

  Scenario: The magnitude of a normalized vector
    Given vnorm2 <- vector(1, 2, 3)
    When norm <- normalize(vnorm2)
    Then magnitude(norm) == 1

  Scenario: The dot product of two tuples
    Given tadot <- vector(1, 2, 3)
    And tbdot <- vector(2, 3, 4)
    Then dot(tadot, tbdot) == 20

  Scenario: the cross product of two vectors
    Given across <- vector(1, 2, 3)
    And bcross <- vector(2, 3, 4)
    Then cross(across, bcross) == vector(-1, 2, -1)
    Then cross(bcross, across) == vector(1, -2, 1)

