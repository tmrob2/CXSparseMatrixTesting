#![allow(non_snake_case)]
mod c_binding;
use crate::c_binding::sparse_clib::*;
extern crate lapacke;
use lapacke::{dgesv, Layout};

pub fn create_sparse_matrix(m: i32, n: i32, rows: &mut [i32], cols: &mut [i32], x: &mut [f64])
    -> *mut cs_di {
    unsafe {
        let T: *mut cs_di = cs_di_spalloc(m, n, x.len() as i32, 1, 1);
        for (k, elem) in x.iter().enumerate() {
            cs_di_entry(T, rows[k], cols[k], *elem);
        }
        return T
    }
}

pub fn convert_to_compressed(T: *mut cs_di) -> *mut cs_di {
    unsafe {
        cs_di_compress(T)
    }
}

pub fn print_matrix(A: *mut cs_di) {
    unsafe {
        cs_di_print(A, 0);
    }
}

pub fn transpose(A: *mut cs_di, nnz: i32) -> *mut cs_di {
    unsafe {
        cs_di_transpose(A, nnz)
    }
}

pub fn multAxA(A: *mut cs_di, B: *mut cs_di) -> *mut cs_di {
    unsafe {
        cs_di_multiply(A, B)
    }
}

pub fn mnorm(A: *mut cs_di) -> f64 {
    unsafe {
        cs_di_norm(A)
    }
}

pub fn solve(A: *mut cs_di, b: &mut [f64], order: i32, tol: f64) {
    unsafe {
        cs_di_lusol(order, A, b.as_mut_ptr(), tol);
    }
}

pub fn solve_qr(A: *mut cs_di, b: &mut [f64], order: i32) {
    unsafe {
        cs_di_qrsol(order, A, b.as_mut_ptr());
    }
}

pub fn spfree(A: *mut cs_di) {
    unsafe {
        cs_di_spfree(A);
    }
}

pub fn solve_lu(A: *mut cs_di, b: &mut [f64], order: i32, tol: f64) {
    let mut x: Vec<f64> = vec![0.; b.len()];
    unsafe {
        let S = cs_di_sqr(order, A, 0);
        let N = cs_di_lu(A, S, tol);
        cs_di_ipvec((*N).pinv, b.as_ptr(), x.as_mut_ptr(), b.len() as i32);
        cs_di_lsolve((*N).L, x.as_mut_ptr());
        cs_di_usolve((*N).U, x.as_mut_ptr());
        cs_di_ipvec((*S).q, x.as_ptr(), b.as_mut_ptr(), b.len() as i32);
    }
}

/// Assume A is in col major fmt
/// Assume matrix is always square
pub fn solve_dense_lu(A: &mut [f64], b: &mut [f64], n: i32, nrhs: i32) {
    let mut ipiv = [n];
    unsafe {
        dgesv(
            Layout::ColumnMajor,
            n,
            nrhs,
            A,
            n,
            &mut ipiv[..],
            b,
            nrhs
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::create_sparse_matrix;

    #[test]
    fn it_works() {
        let mut i: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 4, 4, 4];
        let mut j: Vec<i32> = vec![0, 1, 0, 2, 4, 1, 2, 3, 2, 1, 2, 4];
        let mut vals: Vec<f64> = vec![2., 3., 3., -1., 4., 4., -3., 1., 2., 2., 6., 1.];
        let T = create_sparse_matrix(
            5, &mut i[..], &mut j[..], &mut vals[..]);

    }
}
