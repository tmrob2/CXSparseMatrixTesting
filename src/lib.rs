#![allow(non_snake_case)]
mod c_binding;

use std::collections::HashMap;
use std::fs;
use crate::c_binding::sparse_clib::*;
extern crate lapacke;
use lapacke::{dgesv, Layout};
use serde::{Serialize, Deserialize};
use serde_yaml::{self};
use std::io::{BufWriter, Write};

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

//pub fn sparse_transpose(A: &Sparse, nnz: i32) -> Sparse {
//    let m = A.m;
//    let n = A.n;
//    let cs_A = sparse_to_cs(A);
//    let AT = transpose(cs_A, nnz);
//    unsafe {
//        cs_di_spfree(cs_A);
//    }
//    //cs_to_rust_and_destroy(AT, nnz, m, n)
//}

pub fn free_sparse(A: *mut cs_di_sparse) {
    unsafe {
        cs_di_spfree(A);
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

pub fn spalloc(m: i32, n: i32, nzmax: i32, values: i32, t: i32) -> *mut cs_di {
    unsafe {
        cs_di_spalloc(m, n, nzmax, values, t)
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

pub struct SparseMatrixComponents {
    pub i: Vec<i32>, // row indices per column
    pub p: Vec<i32>, // column ranges
    pub x: Vec<f64>  // values per column row indices
}

#[derive(Debug, Serialize, Deserialize)]
// We should be able to run Rayon with this structure
pub struct Sparse {
    pub nzmax: i32,
    pub m: i32,
    pub n: i32,
    pub p: Vec<i32>,
    pub i: Vec<i32>,
    pub x: Vec<f64>,
    pub nz: i32,
}

impl Sparse {
    pub fn store_matrix_as_yaml(&self, filename: &str) {
        let path = std::env::current_dir().unwrap().join(filename);
        //let path_str = String::from(path.to_string_lossy());
        match fs::remove_file(path) {
            Ok(_) => {
                // the file has been removed
            }
            Err(_) => {
                // no file exists which is fine
            }
        }
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .expect("Couldn't open file");
        serde_yaml::to_writer(f, &self).unwrap();
    }

    pub fn read_matrix_from_file(filename: &str) -> Sparse {
        //println!("opening: {:?}", filename);
        let f = std::fs::File::open(filename).expect("Error opening file");
        let sparse: Sparse = serde_yaml::from_reader(f).expect("Could not read yaml into sparse");
        sparse
    }
}

#[derive(Debug, Serialize, Deserialize)]
// We should be able to run Rayon with this structure
pub struct COO {
    pub nzmax: i32,
    pub m: i32,
    pub n: i32,
    pub i: Vec<i32>,
    pub j: Vec<i32>,
    pub x: Vec<f64>,
    pub nz: i32,
}

impl COO {
    pub fn store_matrix_as_yaml(&self, filename: &str) {
        let mut path = std::env::var("SCPM_TEST").unwrap();
        path.push_str(filename);
        //let path_str = String::from(path.to_string_lossy());
        match fs::remove_file(path.as_str()) {
            Ok(_) => {
                // the file has been removed
            }
            Err(_) => {
                // no file exists which is fine
            }
        }
        let mut f = BufWriter::new(std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.as_str())
            .expect("Couldn't open file"));
        serde_yaml::to_writer(f, &self).unwrap();
        println!("finished writing: {}", filename);
        f.flush().unwrap();
    }

    pub fn read_matrix_from_file(filename: &str) -> COO {
        println!("opening: {:?}", filename);
        let f = std::fs::File::open(filename).expect("Error opening file");
        let triple: COO = serde_yaml::from_reader(f).expect("Could not read yaml into sparse");
        triple
    }
}

pub fn sparse_to_cs2(sparse: &mut Sparse) -> *mut cs_di {
    let A = spalloc(sparse.m, sparse.n, sparse.nzmax, sparse.nz, 0);
    unsafe {
        (*A).i = sparse.i.as_mut_ptr();
        (*A).i = sparse.i.as_mut_ptr();
        (*A).p = sparse.p.as_mut_ptr();
        (*A).x = sparse.x.as_mut_ptr();
    }
    print_matrix(A);
    A
}

pub fn sparse_to_cs3(sparse: &mut COO) -> *mut cs_di {
    todo!()
}

pub fn sparse_to_cs(sparse: &Sparse) -> *mut cs_di {
    //create_sparse_matrix(sparse.m, sparse.n, sparse.)
    let (mut i, mut j, mut x) =
        compressed_to_triple(sparse.n as usize, &sparse.p[..], &sparse.i[..], &sparse.x[..]);
    let T =
        create_sparse_matrix(sparse.m, sparse.n, &mut i[..], &mut j[..], &mut x[..]);
    let A = convert_to_compressed(T);
    A
}

pub fn cs_to_rust_and_destroy(A: *mut cs_di, nnz: i32, m: i32, n: i32) -> Sparse {
    let x: Vec<f64>;
    let p: Vec<i32>;
    let i: Vec<i32>;
    unsafe {
        x = Vec::from_raw_parts((*A).x as *mut f64, nnz as usize, nnz as usize);
        i = Vec::from_raw_parts((*A).i as *mut i32, nnz as usize, nnz as usize);
        p = Vec::from_raw_parts((*A).p as *mut i32, m as usize, m as usize);
        //println!("Deconstruction:\ni: {:?}\np: {:?}\nx: {:?}", i, p, x);
        //cs_di_spfree(A);
    }
    Sparse {
        nzmax: nnz + 1,
        m,
        n,
        p,
        i,
        x,
        nz: nnz
    }
}

/// n: number of cols
pub fn compressed_to_triple(n: usize, p: &[i32], i: &[i32], x: &[f64]) -> (Vec<i32>, Vec<i32>, Vec<f64>) {
    let mut ii: Vec<i32> = Vec::new();
    let mut jj: Vec<i32> = Vec::new();
    let mut vals: Vec<f64> = Vec::new();
    for k in 0..n { // for each column of the tranpose matrix
        for r in p[k]..p[k +1] {
            // for each row recorded in the sparse coord list for column k
            ii.push(i[r as usize]);
            jj.push(k as i32);
            vals.push(x[r as usize]);
        }
    }
    (ii, jj, vals)
}

pub fn deconstruct(A: *mut cs_di, nnz: usize, cols: usize) -> SparseMatrixComponents {
    let x: Vec<f64>;
    let p: Vec<i32>;
    let i: Vec<i32>;
    unsafe {
        x = Vec::from_raw_parts((*A).x as *mut f64, nnz, nnz);
        i = Vec::from_raw_parts((*A).i as *mut i32, nnz, nnz);
        p = Vec::from_raw_parts((*A).p as *mut i32, cols, cols);
        println!("Deconstruction:\ni: {:?}\np: {:?}\nx: {:?}", i, p, x);
    }
    SparseMatrixComponents {i, p, x}
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub struct Meta {
    pub A: Sparse,
    pub a: i32,
    pub t: i32
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
