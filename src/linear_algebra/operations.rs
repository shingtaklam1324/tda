use nalgebra::{Matrix, ComplexField, Dim, RawStorageMut, DMatrix};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// Elementary operations on matrices over type T
pub enum Operation<T> {
    /// Swap columns/row i and j
    Swap(usize, usize),
    /// Replace column/row i with lambda * column/row i
    Scale(usize, T),
    /// Replace column/row i with column/row i + lambda * column/row j
    Add(usize, usize, T),
}

impl<T> Operation<T> where T: ComplexField + Copy {
    pub fn row_op<M: Dim, N: Dim, S: RawStorageMut<T, M, N>>(&self, d: &mut Matrix<T, M, N, S>) {
        match self {
            Operation::Swap(i, j) => d.swap_rows(*i, *j),
            Operation::Scale(i, lambda) => d.row_mut(*i)
                                                      .iter_mut()
                                                      .for_each(|x| *x *= *lambda),
            Operation::Add(i, j, lambda) => {
                for k in 0..d.ncols() {
                    let r = d[(*j, k)];
                    d[(*i, k)] += r * *lambda
                }
            },
        }
    }

    pub fn col_op<M: Dim, N: Dim, S: RawStorageMut<T, M, N>>(&self, d: &mut Matrix<T, M, N, S>) {
        match self {
            Operation::Swap(i, j) => d.swap_columns(*i, *j),
            Operation::Scale(i, lambda) => d.column_mut(*i)
                                                      .iter_mut()
                                                      .for_each(|x| *x *= *lambda),
            Operation::Add(i, j, lambda) => {
                for k in 0..d.nrows() {
                    let r = d[(k, *j)];
                    d[(k, *i)] += r * *lambda
                }
            },
        }
    }

    pub fn inv(&self) -> Self {
        match self {
            Operation::Swap(i, j) => Operation::Swap(*j, *i),
            Operation::Scale(i, lambda) => Operation::Scale(*i, T::one() / *lambda),
            Operation::Add(i, j, lambda) => Operation::Add(*i, *j, -*lambda),
        }
    }

    pub fn row_matrix(&self, n: usize) -> DMatrix<T> {
        let mut m = DMatrix::identity(n, n);
        self.row_op(&mut m);
        m
    }

    pub fn col_matrix(&self, n: usize) -> DMatrix<T> {
        let mut m = DMatrix::identity(n, n);
        self.col_op(&mut m);
        m
    }
}

#[derive(Debug)]
pub struct Operations<T> {
    ops: Vec<Operation<T>>,
}

impl<T> Operations<T> where T: ComplexField + Copy {
    pub fn new() -> Operations<T> {
        Operations {
            ops: Vec::new(),
        }
    }

    pub fn row_op<M: Dim, N: Dim, S: RawStorageMut<T, M, N>>(&self, d: &mut Matrix<T, M, N, S>) {
        self.ops.iter()
            .for_each(|op| op.row_op(d));
    }

    pub fn col_op<M: Dim, N: Dim, S: RawStorageMut<T, M, N>>(&self, d: &mut Matrix<T, M, N, S>) {
        self.ops.iter()
            .for_each(|op| op.col_op(d));
    }

    pub fn push(&mut self, op: Operation<T>) {
        self.ops.push(op);
    }

    pub fn inv(&self) -> Self {
        let mut ops = self.ops.clone();
        ops.reverse();
        ops.iter_mut().for_each(|op| *op = op.inv());
        Operations { ops }
    }

    pub fn row_matrix(&self, n: usize) -> DMatrix<T> {
        let mut m = DMatrix::identity(n, n);
        self.row_op(&mut m);
        m
    }

    pub fn col_matrix(&self, n: usize) -> DMatrix<T> {
        let mut m = DMatrix::identity(n, n);
        self.col_op(&mut m);
        m
    }

}
