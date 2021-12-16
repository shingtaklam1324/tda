use tda::linear_algebra::smith::Smith;
use nalgebra::{DMatrix, dmatrix};

#[test]
fn identity() {
    let m = dmatrix![
        1., 0., 0.;
        0., 1., 0.;
        0., 0., 1.
    ];
    let sm = Smith::of(&m);
    println!("{:?}", sm);
}

#[test]
fn some_matrix1() {
    let m = dmatrix![
        1., 1., 1.;
        0., 1., 1.;
        0., 0., 1.
    ];
    let sm = Smith::of(&m);
    println!("{}", sm.result());
    // println!("{}", sm.col_matrix() * sm.result() * sm.row_matrix());
}

#[test]
fn some_matrix2() {
    let m = dmatrix![
        1., 1., 1.;
        0., 2., 1.;
        0., 0., 1.
    ];
    let sm = Smith::of(&m);
    println!("{}", sm.result());
    let mut d = sm.result().clone();
    sm.row_ops().inv().row_op(&mut d);
    sm.col_ops().inv().col_op(&mut d);
    println!("{}", d);
}

#[test]
fn some_matrix3() {
    let m = dmatrix![
        1., 1., 1.;
        7., 2., 1.;
        8., 6., 3.
    ];
    let sm = Smith::of(&m);
    println!("{}", sm.result());
    let mut d = sm.result().clone();
    sm.row_ops().inv().row_op(&mut d);
    sm.col_ops().inv().col_op(&mut d);
    println!("{}", d);
}

#[test]
fn some_matrix4() {
    let m = dmatrix![
        1., 1., 0.;
        1., 1., 0.;
        0., 0., 1.
    ];
    let sm = Smith::of(&m);
    println!("{}", sm.result());
    let mut d = sm.result().clone();
    sm.row_ops().inv().row_op(&mut d);
    sm.col_ops().inv().col_op(&mut d);
    println!("{}", d);
}
