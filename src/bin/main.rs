#![allow(non_snake_case)]

use std::time::Instant;
use sparse::*;
use rand::prelude::*;
use rayon::prelude::*;

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
    let m: usize = 5; // m is the original number of rows
    let n: usize = 4; // n is the original number of columns 
    //
    for k in 0..n { // cols
        for l in 0..m { // rows
            if k == l {
                i1.push(l as i32);
                j1.push(k as i32);
                val1.push(1.0);
                i2.push(l as i32);
                j2.push(k as i32);
                val2.push(2.0);
            }
        }
    }
    let nnz1 = val1.len();
    let nnz2 = val2.len();

    let T1 = create_sparse_matrix(m as i32, n as i32, &mut i1[..], &mut j1[..], &mut val1[..]);
    let T2 = create_sparse_matrix(m as i32, n as i32, &mut i2[..], &mut j2[..], &mut val2[..]);
    let A1 = convert_to_compressed(T1);
    let A2 = convert_to_compressed(T2);

    // Ok now we have the sparse matrices we can print one and make sure it is what is expected
    print_matrix(A1);
    print_matrix(A2);
    // Now we require the transpose of the matrix
    let AT1 = transpose(A1, nnz1 as i32);
    let AT2 = transpose(A2, nnz2 as i32);
    // At this point we need to deconstruct the tranpose matrices into their component parts in compressed col fmt
    let Acomp1 = deconstruct(AT1, nnz1, n + 1);
    let Acomp2 = deconstruct(AT2, nnz2, n + 1);

    // Initialise the matrix triple
    let mut argmax_i: Vec<i32> = Vec::new();
    let mut argmax_j: Vec<i32> = Vec::new();
    let mut argmax_vals: Vec<f64> = Vec::new();
    // now I need to get rows from the original matriccies and create an 'argmax' matrix from this
    // Generate a random vector which represents a policy
    let actions_space: [i32; 2] = [0, 1];
    let mut rng = thread_rng();
    let rnd: Vec<i32> = (0..n).map(|_| *actions_space.choose(&mut rng).unwrap() ).collect();
    for k in 0..n { // for each column of the tranpose matrix
        let matrix_components: Option<&SparseMatrixComponents> = match rnd[k] {
            0 => Some(&Acomp1),
            1 => Some(&Acomp2),
            _ => None
        };
        let p = &matrix_components.as_ref().unwrap().p;
        let i = &matrix_components.as_ref().unwrap().i;
        let x = &matrix_components.as_ref().unwrap().x;
        println!("matrix components");
        println!("i: {:?}", i);
        println!("p: {:?}", p);
        println!("x: {:?}", x);
    
        for r in p[k]..p[k +1] { // for each row recorded in the sparse coord list for column k
            println!("row: {}, col: {}, vals: {}", i[r as usize], k, x[r as usize]);
            argmax_j.push(i[r as usize]);
            argmax_i.push(k as i32);
            argmax_vals.push(x[r as usize]);
        }
    }

    let argmaxT = create_sparse_matrix(m as i32, n as i32, &mut argmax_i[..], &mut argmax_j[..], &mut argmax_vals[..]);
    print_matrix(argmaxT);
    let argmaxA = convert_to_compressed(argmaxT);
    print_matrix(argmaxA);

    let A = cs_to_rust_and_destroy(argmaxA, argmax_vals.len() as i32, (m + 1) as i32, n as i32);

    let data = Data {
        x: A,
    };

    //sparse_transpose(&data.x, data.x.nz);
    //for mat in r.iter() {
    //    print_matrix(sparse_to_cs(mat))
    //}
    let t1 = Instant::now();
    data.x.store_matrix_as_yaml("test_mat_store.yml");
    println!("time to write matrix: {}", t1.elapsed().as_secs_f64());

    let t1 = Instant::now();
    let S = Sparse::read_matrix_from_file("test_mat_store.yml");
    println!("time to write matrix: {}", t1.elapsed().as_secs_f64());

}