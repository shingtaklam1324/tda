use tda::simplicial_complex::{SimplicialComplex};
use std::collections::BTreeSet;

#[test]
fn make_empty() {
    let k: SimplicialComplex<i32> = SimplicialComplex::new(BTreeSet::new());
    assert_eq!(k.vertices, BTreeSet::new());
    assert_eq!(k.simplices, BTreeSet::new());
}

#[test]
fn make_one_simplex() {
    let k = SimplicialComplex::from(vec![vec![0]]);
    assert_eq!(k.vertices, vec![0].into_iter().collect());
    assert_eq!(k.simplices, vec![vec![0].into()].into_iter().collect());
}

#[test]
fn make_two_simplex1() {
    let k = SimplicialComplex::from(vec![vec![0], vec![1]]);
    assert_eq!(k.vertices, vec![0, 1].into_iter().collect());
    assert_eq!(k.simplices, vec![vec![0].into(), vec![1].into()].into_iter().collect());
}

#[test]
fn make_two_simplex2() {
    let k = SimplicialComplex::from(vec![vec![0], vec![1], vec![0, 1]]);
    assert_eq!(k.vertices, vec![0, 1].into_iter().collect());
    assert_eq!(k.simplices, vec![vec![0], vec![1], vec![0, 1]].into_iter().map(|x| x.into()).collect());
}

#[test]
fn solid_0() {
    let k = SimplicialComplex::solid(0);
    assert_eq!(k, SimplicialComplex::from(vec![vec![0]]));
}

#[test]
fn solid_1() {
    let k = SimplicialComplex::solid(1);
    assert_eq!(k, SimplicialComplex::from(vec![vec![0], vec![1], vec![0, 1]]));
}

#[test]
fn solid_2() {
    let k = SimplicialComplex::solid(2);
    assert_eq!(k, SimplicialComplex::from(vec![vec![0], vec![1], vec![2], vec![0, 1], vec![0, 2], vec![1, 2], vec![0, 1, 2]]));
}

#[test]
fn hollow_1() {
    let k = SimplicialComplex::hollow(1);
    assert_eq!(k, SimplicialComplex::from(vec![vec![0], vec![1]]));
}

#[test]
fn hollow_2() {
    let k = SimplicialComplex::hollow(2);
    assert_eq!(k, SimplicialComplex::from(vec![vec![0], vec![1], vec![2], vec![0, 1], vec![0, 2], vec![1, 2], ]));
}
