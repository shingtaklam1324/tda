use tda::simplicial_complex::SimplicialComplex;
use nalgebra::{DMatrix, dmatrix};

#[test]
fn boundary_solid_0_simplex() {
    let k = SimplicialComplex::solid(0);
    println!("{:?}", k.boundary::<i32>(1));
}

#[test]
fn boundary_solid_1_simplex() {
    let k = SimplicialComplex::solid(1);
    // There is only one non-zero boundary map for the 1-simplex.
    assert_eq!(k.boundary::<i32>(1),
    dmatrix!
       // 01
        [ -1; // 0
           1  // 1
        ]);
}

#[test]
fn boundary_solid_2_simplex() {
    let k = SimplicialComplex::solid(2);
    let d2 = k.boundary::<i32>(2);
    let d1 = k.boundary::<i32>(1);
    // Here, we have two non-zero boundary maps
    assert_eq!(d2,
        dmatrix!
        // 012
        [  1;    // 01
          -1;    // 02
           1]);  // 12
    assert_eq!(d1,
        dmatrix!
        // 01   02   12 
        [  -1, -1,  0;    // 0
            1,  0, -1;    // 1
            0,  1,  1]);  // 2
    // and the composition of boundary maps is zero
    assert_eq!(d1 * d2, DMatrix::zeros(3, 1));
}

#[test]
fn boundary_solid_3_simplex() {
    let k = SimplicialComplex::solid(3);
    let d3 = k.boundary(3);
    let d2 = k.boundary(2);
    let d1 = k.boundary(1);
    // Here, we have three non-zero boundary maps
    assert_eq!(d3,
    dmatrix!
        // 0123
         [ -1;    // 012
            1;    // 013
           -1;    // 023
            1]);  // 123
    assert_eq!(d2,
    dmatrix!
        // 012  013  023  123
         [  1,    1,   0,   0; // 01
           -1,    0,   1,   0; // 02
            0,   -1,  -1,   0; // 03
            1,    0,   0,   1; // 12
            0,    1,   0,  -1; // 13
            0,    0,   1,   1; // 23
           ]);
    assert_eq!(d1,
    dmatrix!
        // 01  02  03  12  13  23
         [ -1, -1, -1,  0,  0,  0; // 0
            1,  0,  0, -1, -1,  0; // 1
            0,  1,  0,  1,  0, -1; // 2
            0,  0,  1,  0,  1,  1; // 3
        ]);
    // and the composition of adjacent boundary maps is zero
    assert_eq!(d1 * &d2, DMatrix::zeros(4, 4));
    assert_eq!(d2 * d3, DMatrix::zeros(6, 1));
}

#[test]
fn betti_solid_0_simplex() {
    let k = SimplicialComplex::solid(0);
    // First 4 Betti numbers of the solid 0-simplex
    assert_eq!(k.betti(0), 1);
    assert_eq!(k.betti(1), 0);
    assert_eq!(k.betti(2), 0);
    assert_eq!(k.betti(3), 0);
}

#[test]
fn betti_solid_1_simplex() {
    let k = SimplicialComplex::solid(1);
    // First 4 Betti numbers of the solid 1-simplex
    assert_eq!(k.betti(0), 1);
    assert_eq!(k.betti(1), 0);
    assert_eq!(k.betti(2), 0);
    assert_eq!(k.betti(3), 0);
}

#[test]
fn betti_solid_2_simplex() {
    let k = SimplicialComplex::solid(2);
    // First 4 Betti numbers of the solid 2-simplex
    assert_eq!(k.betti(0), 1);
    assert_eq!(k.betti(1), 0);
    assert_eq!(k.betti(2), 0);
    assert_eq!(k.betti(3), 0);
}

#[test]
fn betti_hollow_1_simplex() {
    let k = SimplicialComplex::hollow(1);
    // First 4 Betti numbers of the hollow 1-simplex
    assert_eq!(k.betti(0), 2);
    assert_eq!(k.betti(1), 0);
    assert_eq!(k.betti(2), 0);
    assert_eq!(k.betti(3), 0);
}

#[test]
fn betti_hollow_2_simplex() {
    let k = SimplicialComplex::hollow(2);
    // First 4 Betti numbers of the hollow 2-simplex
    assert_eq!(k.betti(0), 1);
    assert_eq!(k.betti(1), 1);
    assert_eq!(k.betti(2), 0);
    assert_eq!(k.betti(3), 0);
}
