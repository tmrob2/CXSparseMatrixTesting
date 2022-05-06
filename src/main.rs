#![allow(non_snake_case)]

use std::mem;
use sparse::*;

fn main() {
    //let mut i: Vec<i32> = vec![2, 1, 3, 0, 1, 3, 3, 1, 0, 2];
    //let mut j: Vec<i32> = vec![2, 0, 3, 2, 1, 0, 1, 3, 0, 1];
    //let mut vals: Vec<f64> = vec![3.0, 3.1, 1.0, 3.2, 2.9, 3.5, 0.4, 0.9, 4.5, 1.7];
    let mut i: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 4, 4, 4];
    let mut j: Vec<i32> = vec![0, 1, 0, 2, 4, 1, 2, 3, 2, 1, 2, 4];
    let mut vals: Vec<f64> = vec![2., 3., 3., 4., 6., -1., -3., 2., 1., 4., 2., 1.];
    //let T = create_sparse_matrix(4, &mut i[..], &mut j[..], &mut vals[..]);
    //let A = convert_to_compressed(T);
    //print_matrix(A);
    //print_matrix(T);
    //let AT = transpose(A, 10);
    //print_matrix(AT);
    //let I = multAxA(A, AT);
    //print_matrix(I)
    let T = create_sparse_matrix(5, &mut i[..], &mut j[..], &mut vals[..]);
    let A = convert_to_compressed(T);
    let x: Vec<f64>;
    let p: Vec<i32>;
    let i: Vec<i32>;
    let xt: Vec<f64>;
    let pt: Vec<i32>;
    unsafe {
        let AT = transpose(A, 1);
        print_matrix(A);
        x = Vec::from_raw_parts((*A).x as *mut f64, vals.len(), vals.len());
        i = Vec::from_raw_parts((*A).i as *mut i32, vals.len(), vals.len());
        p = Vec::from_raw_parts((*A).p as *mut i32, 6, 6);
        print_matrix(AT);
        xt = Vec::from_raw_parts((*AT).x as *mut f64, vals.len(), vals.len());
        pt = Vec::from_raw_parts((*AT).p as *mut i32, 6, 6);
        //mem::forget(A);
    };
    println!("x: {:?}", x);
    println!("p: {:?}", p);
    println!("i: {:?}", i);
    println!("x: {:?}", xt);
    println!("p: {:?}", pt);

    // we want to create a new sparse matrix from the components of the old matrix, specifically the
    // transpose
    let mut ii2: Vec<i32> = Vec::with_capacity(vals.len());
    let mut jj2: Vec<i32> = Vec::with_capacity(vals.len());
    let mut vals2: Vec<f64> = Vec::with_capacity(vals.len());
    for k in 0..5 {
        for r in p[k]..p[k +1] {
            println!("row: {}, col: {}, vals: {}", i[r as usize], k, x[r as usize]);
            ii2.push(i[r as usize]);
            jj2.push(k as i32);
            vals2.push(x[r as usize]);
        }
    }
    let T2 = create_sparse_matrix(5, &mut ii2[..], &mut jj2[..], &mut vals2[..]);
    let A2 = convert_to_compressed(T2);
    print_matrix(A2);

    //print_matrix(A);

    //let mut b: Vec<f64> = vec![8., 45., -3., 3., 19.];
    //solve(A, &mut b[..], 0, 1e-5);
    //println!("b: {:?}", b);
    //let mut b: Vec<f64> = vec![8., 45., -3., 3., 19.];
    //solve_lu(A, &mut b[..], 0, 1e-5);
    //println!("b with LU: {:?}", b);
    //let mut b: Vec<f64> = vec![8., 45., -3., 3., 19.];
    //solve_qr(A, &mut b[..], 0);
    //println!("b with QR: {:?}", b);
    //// Solve with lapack
    //// Specify matrix in column major format
    //spfree(A);
    //let mut A = vec![
    //    2., 3., 0., 0., 0.,
    //    3., 0., -1., 0., 4.,
    //    0., 4., -3., 1., 2.,
    //    0., 0., 2., 0., 0.,
    //    0., 6., 0., 0., 1.
    //];
    //let mut b: Vec<f64> = vec![8., 45., -3., 3., 19.];
    //solve_dense_lu(&mut A[..], &mut b[..], 5, 5);
    //println!("b: {:?}", b);

}