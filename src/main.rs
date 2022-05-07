#![allow(non_snake_case)]

use sparse::*;
use rand::prelude::*;

fn main() {

    // construct a matrix of 1s, 2s, and we will task every second row from 2
    // the matrix dimensions will also need to be asymtetric
    let mut i1: Vec<i32> = Vec::new();
    let mut j1: Vec<i32> = Vec::new();
    let mut val1: Vec<f64> = Vec::new();
    //
    let mut i2: Vec<i32> = Vec::new();
    let mut j2: Vec<i32> = Vec::new();
    let mut val2: Vec<f64> = Vec::new();
    //
    let m: usize = 5;
    let n: usize = 4;
    //
    for k in 0..m {
        for l in 0..n {
            if k == l {
                i1.push(l as i32);
                j1.push(k as i32);
                val1.push(1.0);

                i2.push(l as i32);
                j2.push(l as i32);
                val2.push(2.0);
            }
        }
    }

    let T1 = create_sparse_matrix(m as i32, n as i32, &mut i1[..], &mut j1[..], &mut val1[..]);
    let T2 = create_sparse_matrix(m as i32, n as i32, &mut i2[..], &mut j2[..], &mut val2[..]);
    let A1 = convert_to_compressed(T1);
    let A2 = convert_to_compressed(T2);

    // Ok now we have the sparse matrices we can print one and make sure it is what is expected
    print_matrix(A1);
    print_matrix(A2);
    // Initialise the matrix triple
    let mut argmax_i: Vec<i32> = Vec::new();
    let mut argmax_j: Vec<i32> = Vec::new();
    let mut argmax_vals: Vec<i32> = Vec::new();
    // now I need to get rows from the original matriccies and create an 'argmax' matrix from this
    // Generate a random vector which represents a policy
    let actions_space: [i32; 2] = [0, 1];
    let mut rng = thread_rng();
    let rnd: Vec<i32> = (0..m).map(|_| *actions_space.choose(&mut rng).unwrap() ).collect();
    for k in 0..5 {
        for r in p[k]..p[k +1] {
            println!("row: {}, col: {}, vals: {}", i[r as usize], k, x[r as usize]);
            argmax_j.push(i[r as usize]);
            argmax_i.push(k as i32);
            argmax_vals.push(x[r as usize]);
        }
    }

    let argmaxT = create_sparse_matrix(n, m, &argmax_i[..], &argmax_j[..], &argmax_vals[..]);
    let argmaxA = convert_to_compressed(argmaxT);

    print_matrix(argmaxA);

    //let mut i: Vec<i32> = vec![2, 1, 3, 0, 1, 3, 3, 1, 0, 2];
    //let mut j: Vec<i32> = vec![2, 0, 3, 2, 1, 0, 1, 3, 0, 1];
    //let mut vals: Vec<f64> = vec![3.0, 3.1, 1.0, 3.2, 2.9, 3.5, 0.4, 0.9, 4.5, 1.7];
    //let mut i: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 4, 4, 4];
    //let mut j: Vec<i32> = vec![0, 1, 0, 2, 4, 1, 2, 3, 2, 1, 2, 4];
    //let mut vals: Vec<f64> = vec![2., 3., 3., 4., 6., -1., -3., 2., 1., 4., 2., 1.];
    //let T = create_sparse_matrix(4, &mut i[..], &mut j[..], &mut vals[..]);
    //let A = convert_to_compressed(T);
    //print_matrix(A);
    //print_matrix(T);
    //let AT = transpose(A, 10);
    //print_matrix(AT);
    //let I = multAxA(A, AT);
    //print_matrix(I)
    //let T = create_sparse_matrix(5, &mut i[..], &mut j[..], &mut vals[..]);
    //let A = convert_to_compressed(T);
    //let x: Vec<f64>;
    //let p: Vec<i32>;
    //let i: Vec<i32>;
    //let xt: Vec<f64>;
    //let pt: Vec<i32>;
    //unsafe {
    //    let AT = transpose(A, 1);
    //    print_matrix(A);
    //    x = Vec::from_raw_parts((*A).x as *mut f64, vals.len(), vals.len());
    //    i = Vec::from_raw_parts((*A).i as *mut i32, vals.len(), vals.len());
    //    p = Vec::from_raw_parts((*A).p as *mut i32, 6, 6);
    //    print_matrix(AT);
    //    xt = Vec::from_raw_parts((*AT).x as *mut f64, vals.len(), vals.len());
    //    pt = Vec::from_raw_parts((*AT).p as *mut i32, 6, 6);
    //    //mem::forget(A);
    //};
    //println!("x: {:?}", x);
    //println!("p: {:?}", p);
    //println!("i: {:?}", i);
    //println!("x: {:?}", xt);
    //println!("p: {:?}", pt);

    // we want to create a new sparse matrix from the components of the old matrix, specifically the
    // transpose
    //let mut ii2: Vec<i32> = Vec::with_capacity(vals.len());
    //let mut jj2: Vec<i32> = Vec::with_capacity(vals.len());
    //let mut vals2: Vec<f64> = Vec::with_capacity(vals.len());
    //for k in 0..5 {
    //    for r in p[k]..p[k +1] {
    //        println!("row: {}, col: {}, vals: {}", i[r as usize], k, x[r as usize]);
    //        ii2.push(i[r as usize]);
    //        jj2.push(k as i32);
    //        vals2.push(x[r as usize]);
    //    }
    //}
    //let T2 = create_sparse_matrix(5, &mut ii2[..], &mut jj2[..], &mut vals2[..]);
    //let A2 = convert_to_compressed(T2);
    //print_matrix(A2);

    //// construct a tranpose matrix
    //let AT2 = transpose(A2, vals2.len());
    //let i2: Vec<i32>;
    //let p2: Vec<i32>;
    //let x2: Vec<f64>;

    //unsafe {
    //    let AT = transpose(A, 1);
    //    x2 = Vec::from_raw_parts((*A).x as *mut f64, vals.len(), vals.len());
    //    i2 = Vec::from_raw_parts((*A).i as *mut i32, vals.len(), vals.len());
    //    p2 = Vec::from_raw_parts((*A).p as *mut i32, 6, 6);
    //};

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