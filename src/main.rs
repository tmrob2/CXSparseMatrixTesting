#![allow(non_snake_case)]
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
    let mut b: Vec<f64> = vec![8., 45., -3., 3., 19.];
    solve(A, &mut b[..], 0, 1e-5);
    println!("b: {:?}", b);
    let mut b: Vec<f64> = vec![8., 45., -3., 3., 19.];
    solve_lu(A, &mut b[..], 0, 1e-5);
    println!("b with LU: {:?}", b);
    let mut b: Vec<f64> = vec![8., 45., -3., 3., 19.];
    solve_qr(A, &mut b[..], 0);
    println!("b with QR: {:?}", b);
}