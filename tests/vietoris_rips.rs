use tda::vietoris_rips::{vietoris_rips, vietoris_rips_step};
use tda::simplicial_complex::SimplicialComplex;

use nalgebra::dmatrix;

#[test]
fn single_point() {
    let dist = dmatrix![0.];
    let k0 = vietoris_rips(&dist, 0.);
    assert_eq!(k0, SimplicialComplex::solid(0));
    let k1 = vietoris_rips(&dist, 1.);
    assert_eq!(k0, k1);
}

#[test]
fn two_points() {
    let dist = dmatrix!
        [0., 1.;
         1., 0.];
    let k0 = vietoris_rips(&dist, 0.);
    assert_eq!(k0, SimplicialComplex::hollow(1));
    let k1 = vietoris_rips(&dist, 1.);
    assert_eq!(k1, SimplicialComplex::solid(1));
    let k1step = vietoris_rips_step(&dist, 1., k1.clone());
    assert_eq!(k1, k1step);
}
