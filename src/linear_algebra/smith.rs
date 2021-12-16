use super::operations::{Operation, Operations};
use nalgebra::{DMatrix, ComplexField};

#[derive(Debug)]
pub struct Smith<T> {
    row_ops: Operations<T>,
    col_ops: Operations<T>,
    result: DMatrix<T>
}

impl<T: ComplexField + Copy> Smith<T> {

    pub fn of(d: &DMatrix<T>) -> Self {
        let n = d.nrows().min(d.ncols());
        let mut row_ops = Operations::new();
        let mut col_ops = Operations::new();
        let mut result = d.clone();
        for k in 0..n {
            'row: for i in k..d.nrows() {
                'col: for j in k..d.ncols() {
                    // If (i, j) entry is zero, skip it
                    if result[(i, j)] == T::zero() {
                        continue 'col;
                    }
                    // otherwise, result (i, j) is nonzero
                    // swap it to (k, k)
                    if i != k {
                        let rop = Operation::Swap(i, k);
                        rop.row_op(&mut result);
                        row_ops.push(rop);
                    }
                    if j != k {
                        let cop = Operation::Swap(j, k);
                        cop.col_op(&mut result);
                        col_ops.push(cop);
                    }
                    let t = result[(k, k)];
                    // Rescale entry (k, k) to 1
                    if t != T::one() {
                        let rop = Operation::Scale(k, T::one() / t);
                        row_ops.push(rop);
                        rop.row_op(&mut result);
                    }
                    // Now clear out the k-th row after the (j+1)-th column
                    for l in (k+1)..d.ncols() {
                        let c = result[(k, l)];
                        // If the (k, l) entry is non-zero, then subtract (k, k) from this column
                        if c != T::zero() {
                            let cop = Operation::Add(l, k, -c);
                            cop.col_op(&mut result);
                            col_ops.push(cop);
                        }
                    }
                    // Now clear out the k-th column after the (k+1)-th row
                    for l in (k+1)..d.nrows() {
                        let c = result[(l, k)];
                        // If the (l, k) entry is non-zero, then subtract (k, k) from this row
                        if c != T::zero() {
                            let rop = Operation::Add(l, k, -c);
                            rop.row_op(&mut result);
                            row_ops.push(rop);
                        }
                    }
                    // Then we are done with the k-th row/col
                    continue 'row;
                }
            }
        }
        Self {
            row_ops,
            col_ops,
            result
        }
    }

    pub fn row_ops(&self) -> &Operations<T> {
        &self.row_ops
    }

    pub fn col_ops(&self) -> &Operations<T> {
        &self.col_ops
    }

    pub fn result(&self) -> &DMatrix<T> {
        &self.result
    }

    // Probably wrong?
    // pub fn row_matrix(&self) -> DMatrix<T> {
    //     self.row_ops.row_matrix(self.result.nrows())
    // }

    // pub fn col_matrix(&self) -> DMatrix<T> {
    //     self.col_ops.col_matrix(self.result.ncols())
    // }
}
