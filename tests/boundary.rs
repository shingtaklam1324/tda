use tda::simplicial_complex::SimplicialComplex;
use nalgebra::DMatrix;

#[test]
fn boundary_solid_1_simplex() {
    let k = SimplicialComplex::solid(1);
    // There is only one non-zero boundary map for the 1-simplex.
    assert_eq!(k.boundary(1),
    DMatrix::from_row_slice(2, 1, 
        // 01
        & [-1,    // 0
            1])); // 1
}

#[test]
fn boundary_solid_2_simplex() {
    let k = SimplicialComplex::solid(2);
    let d2 = k.boundary(2);
    let d1 = k.boundary(1);
    // Here, we have two non-zero boundary maps
    assert_eq!(d2,
    DMatrix::from_row_slice(3, 1,
        // 012
        &[ 1,    // 01
          -1,    // 02
           1])); // 12
    assert_eq!(d1,
    DMatrix::from_row_slice(3, 3,
        // 01  02  12 
        &[ -1, -1,  0,    // 0
            1,  0, -1,    // 1
            0,  1,  1])); // 2
    // and the composition of boundary maps is zero
    assert_eq!(d1 * d2,
    DMatrix::from_row_slice(3, 1,
        &[0,
          0,
          0]))
}

#[test]
fn boundary_solid_3_simplex() {
    let k = SimplicialComplex::solid(3);
    let d3 = k.boundary(3);
    let d2 = k.boundary(2);
    let d1 = k.boundary(1);
    // Here, we have three non-zero boundary maps
    assert_eq!(d3,
    DMatrix::from_row_slice(4, 1,
        // 0123
        &[ -1,    // 012
            1,    // 013
           -1,    // 023
            1])); // 123
    assert_eq!(d2,
    DMatrix::from_row_slice(6, 4,
        // 012  013  023  123
        &[  1,    1,   0,   0, // 01
           -1,    0,   1,   0, // 02
            0,   -1,  -1,   0, // 03
            1,    0,   0,   1, // 12
            0,    1,   0,  -1, // 13
            0,    0,   1,   1, // 23
           ]));
    assert_eq!(d1,
        DMatrix::from_row_slice(4, 6,
        // 01  02  03  12  13  23
        &[ -1, -1, -1,  0,  0,  0, // 0
            1,  0,  0, -1, -1,  0, // 1
            0,  1,  0,  1,  0, -1, // 2
            0,  0,  1,  0,  1,  1, // 3
        ]));
    // and the composition of adjacent boundary maps is zero
    assert_eq!(d1 * &d2, DMatrix::zeros(4, 4));
    assert_eq!(d2 * d3, DMatrix::zeros(6, 1));
}
