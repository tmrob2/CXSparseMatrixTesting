#![allow(non_snake_case)]

use std::collections::{HashMap, VecDeque};
use std::{thread, time};
use std::sync::mpsc::{Receiver, RecvError, sync_channel, SyncSender};
use rand::distributions::Uniform;
use rand::prelude::SliceRandom;
use rand::{Rng, thread_rng};
use serde::de::Unexpected::Str;
use sparse::*;

fn main() {
    let num_agents = 3;
    let num_tasks = 2;
    // construct some random matrices
    create_mat_files(&num_tasks, &num_agents);

    let (tx, rx): (SyncSender<Meta>, Receiver<Meta>) = sync_channel(3);
    // if the vecdeque contains less than three matrices load another matrix

    fsend(&num_tasks, &num_agents, &tx);
    // the receiving thread controls doing something with the FIFO queue of matrices,
    // because there is only one sending thread we will always know that the ordering
    // of the matrices will be correct.
    freceive(rx, &num_tasks, &num_agents);
}

fn fsend(num_tasks: &i32, num_agents: &i32, tx: &SyncSender<Meta>) {
    let agents = *num_agents;
    let tasks = *num_tasks;
    let thread_worker = tx.clone();
    thread::spawn(move || {
        let mut count = 0;
        let mut queue: VecDeque<Meta> = VecDeque::new();
        for t in (0..tasks).rev() {
            for a in (0..agents).rev() {
                let filename = &*format!("mat_{}_{}.yml", a, t);
                //println!("Opening: {}", filename);
                let S = Sparse::read_matrix_from_file(filename);
                queue.push_back(Meta{ A: S, a, t });
                println!("loaded mat_{}_{}", a, t);
                count += 1;
                if count >= 3 {
                    // empty the queue
                    while !queue.is_empty() {
                        thread_worker.send(queue.pop_front().unwrap()).unwrap();
                    }
                }
            }
        }
    });
}

fn freceive(rx: Receiver<Meta>, num_tasks: &i32, num_agents: &i32) {
    for t in (0..*num_tasks).rev() {
        for a in (0..*num_agents).rev() {
            // receive a matrix
            match rx.recv() {
                Ok(mut m) => unsafe {
                    let A = sparse_to_cs2(&mut m.A);
                    print_matrix(A);
                    println!("received mat_{}_{}", m.a, m.t);
                    assert_eq!(m.t, t, "task index not equal to expected");
                    assert_eq!(m.a, a, "agent index not equal to expected");
                    // some long running task
                    thread::sleep(time::Duration::from_secs(1));
                    // throw it away: manage memory
                    //free_sparse(A);
                }
                Err(_) => {}
            };

        }
    }
}

fn create_mat_files(num_tasks: &i32, num_agents: &i32) {
    let msize = 4;
    let nsize = 4;
    let j_ind = (0..nsize).collect::<Vec<i32>>();
    let mut rng = thread_rng();
    // store some matrices as yaml files
    for t in 0..*num_tasks {
        for a in 0..*num_agents {
            // check if a matrix already exists
            let filename = &*format!("mat_{}_{}.yml", a, t);
            let mut i: Vec<i32> = Vec::new();
            let mut j: Vec<i32> = Vec::new();
            let mut vals: Vec<f64> = Vec::new();
            for ii in 0..msize {
                for jj in j_ind.choose_multiple(&mut rng, 2) {
                    i.push(ii);
                    j.push(*jj);
                    vals.push(rng.gen());
                }
            }
            let T = create_sparse_matrix(msize, nsize, &mut i[..], &mut j[..], &mut vals[..]);
            let A = convert_to_compressed(T);
            let S = cs_to_rust_and_destroy(A, vals.len() as i32, msize, nsize);
            // now write the matrix to disk
            S.store_matrix_as_yaml(filename);
        }
    }
}