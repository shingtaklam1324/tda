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
            .map(|x| *x)
            .collect();
        SimplicialComplex { vertices, simplices }
    }

    pub fn dim_simplices(&self, k: usize) -> BTreeSet<Simplex<T>> {
        self.simplices.iter()
            .filter(|s| s.dim() == k)
            .cloned()
            .collect()
    }
}

impl<T> From<Vec<Vec<T>>> for SimplicialComplex<T> where T: Ord + Copy {
    fn from(v: Vec<Vec<T>>) -> Self {
        SimplicialComplex::new(v.into_iter()
            .map(|x| x.into())
            .collect())
    }
}

impl SimplicialComplex<i32> {
    pub fn solid(k: usize) -> SimplicialComplex<i32> {
        if k == 0 {
            SimplicialComplex::from(vec![vec![0]])
        } else {
            let mut s = SimplicialComplex::solid(k - 1);
            let mut new_simplicies = s.simplices.iter()
                .map(|s| s.add_vertex(k as i32))
                .collect();
            s.simplices.append(&mut new_simplicies);
            s.simplices.insert(Simplex::from_iter([k as i32]));
            s.vertices.insert(k as i32);
            s
        }
    }

    pub fn hollow(k: usize) -> SimplicialComplex<i32> {
        if k == 0 {
            panic!("Cannot make a hollow simplicial complex of dimension 0");
        }
        let mut s = SimplicialComplex::solid(k);
        s.simplices.remove(&Simplex::from_iter(0..=(k as i32)));
        s
    }
}
