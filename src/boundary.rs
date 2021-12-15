use super::simplicial_complex::SimplicialComplex;
use nalgebra::DMatrix;
use num_traits::Num;
use std::fmt::Debug;

impl<T> SimplicialComplex<T> where T: Ord + Copy {
    /// Compute the k-th boundary map of a simplicial complex.
    pub fn boundary<U: 'static + Num + Clone + Debug>(&self, k: usize) -> DMatrix<U> {
        if k == 0 {
            let n = self.vertices.len();
            return DMatrix::zeros(0, n);
        }
        let ck = self.dim_simplices(k);
        let ck1 = self.dim_simplices(k - 1);
        let n = ck.len();
        let m = ck1.len();
        let i = ck.into_iter()
            .map(|s| ck1.iter()
                .map(move |t| s.boundary_coeff(t)))
                .flatten();
        DMatrix::from_iterator(m, n, i)
    }

    pub fn betti(&self, k: usize) -> usize {
        let dk = self.boundary::<f64>(k);
        let dk1 = self.boundary::<f64>(k + 1);
        let rk = if dk.nrows() * dk.ncols() == 0 { 0 } else { dk.rank(1e-5) };
        let rk1 = if dk1.nrows() * dk1.ncols() == 0 { 0 } else { dk1.rank(1e-5) };
        dk.ncols() - (rk + rk1)
    }

}
