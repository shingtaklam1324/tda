use std::{collections::BTreeSet, iter::FromIterator};
use num_traits::Num;

/// A simplex is a set of vertices, which we assume to be in increasing order.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Simplex<T> {
    pub vertices: BTreeSet<T>,
}

impl<T> Simplex<T> where T: Ord + Clone {
    pub fn new(vertices: BTreeSet<T>) -> Self {
        Simplex { vertices }
    }

    pub fn dim(&self) -> usize {
        self.vertices.len() - 1
    }

    /// s.codim(t) is the codimension of t in s
    pub fn codim(&self, t: &Self) -> usize {
        self.dim() - t.dim()
    }

    /// s.is_face(t) iff t is a face of s, ie. t is a subset of s.
    pub fn is_face(&self, t: &Self) -> bool {
        self.vertices.is_subset(&t.vertices)
    }

    pub fn boundary_coeff<U: Num>(&self, t: &Self) -> U {
        // If it is not a face, or if the codimension is not 1, then we return 0
        if !t.is_face(self) || self.codim(t) != 1 {
            return U::zero();
        // Otherwise, it is (-1)^i, where t is self with the i-th entry removed.
        } else {
            let k = self.vertices
            .iter()
            .enumerate()
            .filter(|&(_, v)| !t.vertices.contains(v))
            .nth(0)
            .unwrap().0;
            if k % 2 == 0 {
                U::one()
            } else {
                U::zero() - U::one()
            }
        }
    }

    pub fn add_vertex(&self, v: T) -> Self {
        let mut b = self.vertices.clone();
        b.insert(v);
        Simplex::new(b)
    }
}

impl<T> From<Vec<T>> for Simplex<T> where T: Ord + Clone {
    fn from(v: Vec<T>) -> Self {
        Simplex::new(v.into_iter().collect())
    }
}

impl<T> FromIterator<T> for Simplex<T> where T: Ord + Clone {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item = T>
    {
        Simplex::new(iter.into_iter().collect())
    }
}
