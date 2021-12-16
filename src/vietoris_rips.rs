use std::collections::BTreeSet;
use std::iter::FromIterator;

use nalgebra::DMatrix;

use super::simplex::Simplex;
use super::simplicial_complex::SimplicialComplex;

fn vr_simplex_step(adj: &DMatrix<bool>, simplices: &BTreeSet<Simplex<usize>>, dim: bool) -> BTreeSet<Simplex<usize>> {
    let mut new_simplices = BTreeSet::new();
    let d = simplices.iter().map(|s| s.dim()).max().unwrap();
    for simplex in simplices {
        if dim && simplex.dim() < d {
            continue;
        }
        let mut with_new_vert = (0..adj.nrows())
            .filter(|&i| simplex.vertices
                                .iter()
                                .all(|&j| adj[(i, j)]))
            .map(|i| simplex.add_vertex(i))
            .collect();
        new_simplices.append(&mut with_new_vert);
    }
    simplices.union(&new_simplices).cloned().collect()
}

fn vr_simplices(adj: &DMatrix<bool>, simplices: BTreeSet<Simplex<usize>>, dim: bool) -> BTreeSet<Simplex<usize>> {
    let mut simplices = simplices;
    loop {
        let new_simplices = vr_simplex_step(&adj, &simplices, dim);
        if new_simplices.len() == simplices.len() {
            break;
        }
        simplices = new_simplices;
    }
    simplices
}

fn vr_simplicial_complex(adj: &DMatrix<bool>) -> SimplicialComplex<usize> {
    // Insert all 0-simplices
    let simplices = (0..adj.nrows()).map(|i| Simplex::from_iter([i])).collect();
    SimplicialComplex::new(vr_simplices(adj, simplices, true))
}

/// Given a distance matrix, and epsilon, return the Vietoris-Rips complex.
pub fn vietoris_rips(dist: &DMatrix<f64>, epsilon: f64) -> SimplicialComplex<usize> {
    let adj = dist.map(|x| x <= epsilon);
    vr_simplicial_complex(&adj)
}

/// Given a distance matrix, epsilon, and the VR complex for some delta < epsilon, return the
/// VR complex for epsilon.
/// 
/// We can do this as any simplex in VR_delta must be a simplex in VR_epsilon.
pub fn vietoris_rips_step(dist: &DMatrix<f64>, epsilon: f64, k: SimplicialComplex<usize>) -> SimplicialComplex<usize> {
    let adj = dist.map(|x| x <= epsilon);
    // When we increase epsilon, we can't only add max dimension simplices, we also need to add
    // lower dimension simplices as well.
    SimplicialComplex::new(vr_simplices(&adj, k.simplices, false))
}
