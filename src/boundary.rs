use super::simplicial_complex::SimplicialComplex;
use nalgebra::DMatrix;

impl<T> SimplicialComplex<T> where T: Ord + Copy {
    /// Compute the k-th boundary map of a simplicial complex.
    pub fn boundary(&self, k: usize) -> DMatrix<i32> {
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
}
