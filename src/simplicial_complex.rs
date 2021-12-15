use std::collections::BTreeSet;
use super::simplex::Simplex;
use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimplicialComplex<T> {
    pub vertices: BTreeSet<T>,
    pub simplices: BTreeSet<Simplex<T>>
}

impl<T> SimplicialComplex<T> where T: Ord + Copy {
    pub fn new(simplices: BTreeSet<Simplex<T>>) -> Self {
        let vertices = simplices.iter()
            .map(|s| &s.vertices)
            .flatten()
            .copied()
            .collect();
        SimplicialComplex { vertices, simplices }
    }

    /// Returns the set of k-simplices in the complex.
    pub fn dim_simplices(&self, k: usize) -> BTreeSet<Simplex<T>> {
        self.simplices.iter()
            .filter(|s| s.dim() == k)
            .cloned()
            .collect()
    }

    /// The dimension of a simplicial complex is defined to be the largest dimension of any simplex.
    pub fn dim(&self) -> usize {
        self.simplices.iter()
            .map(|s| s.dim())
            .max()
            .unwrap_or(0)
    }

    /// Euler characteristic of a simplicial complex is the alternating sum of the 
    /// number of simplices in each dimension.
    pub fn euler(&self) -> isize {
        (0..=self.dim()).enumerate()
            .map(|(i, dim)| (-1isize).pow(i as u32) * 
                self.dim_simplices(dim).len() as isize)
            .sum()
    }
}

impl<T> From<Vec<Vec<T>>> for SimplicialComplex<T> where T: Ord + Copy {
    fn from(v: Vec<Vec<T>>) -> Self {
        SimplicialComplex::new(v.into_iter()
            .map(|x| x.into())
            .collect())
    }
}

impl SimplicialComplex<usize> {
    pub fn solid(k: usize) -> SimplicialComplex<usize> {
        if k == 0 {
            SimplicialComplex::from(vec![vec![0]])
        } else {
            let mut s = SimplicialComplex::solid(k - 1);
            let mut new_simplicies = s.simplices.iter()
                .map(|s| s.add_vertex(k))
                .collect();
            s.simplices.append(&mut new_simplicies);
            s.simplices.insert(Simplex::from_iter([k]));
            s.vertices.insert(k);
            s
        }
    }

    pub fn hollow(k: usize) -> SimplicialComplex<usize> {
        if k == 0 {
            panic!("Cannot make a hollow simplicial complex of dimension 0");
        }
        let mut s = SimplicialComplex::solid(k);
        s.simplices.remove(&Simplex::from_iter(0..=(k)));
        s
    }
}
