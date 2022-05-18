#![allow(non_snake_case)]

use std::collections::{HashMap, VecDeque};
use std::{thread, time};
use std::fmt::format;
use std::sync::mpsc::{Receiver, RecvError, SendError, sync_channel, SyncSender};
use std::time::{Duration, Instant};
use rand::distributions::Uniform;
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::{Rng, thread_rng};
use serde::de::Unexpected::Str;
use sparse::*;
use threadpool::ThreadPool;


fn main() {
    let num_agents = 5;
    let num_tasks = 10;
    // construct some random matrices
    create_mat_files(&num_tasks, &num_agents);

    let (tx, rx): (SyncSender<ChannelData>, Receiver<ChannelData>) = sync_channel(3);
    // if the vecdeque contains less than three matrices load another matrix

    fsend(&num_tasks, &num_agents, &tx);
    // the receiving thread controls doing something with the FIFO queue of matrices,
    // because there is only one sending thread we will always know that the ordering
    // of the matrices will be correct.
    freceive(rx, &num_tasks, &num_agents);
}

fn fsend(num_tasks: &i32, num_agents: &i32, tx: &SyncSender<ChannelData>) {
    let agents = *num_agents;
    let tasks = *num_tasks;
    let thread_worker = tx.clone();
    thread::spawn(move || {
        let mut count = 0;
        let mut queue: VecDeque<ChannelData> = VecDeque::new();
        for t in (0..tasks).rev() {
            for a in (0..agents).rev() {
                let filename = &*format!("mat_{}_{}.yml", a, t);
                let mut path = std::env::var("SCPM_TEST").unwrap();
                path.push_str(filename);
                //println!("Opening: {}", filename);
                let S = COO::read_matrix_from_file(path.as_str());
                queue.push_back(ChannelData{ S, a, t });
                //println!("loaded mat_{}_{}", a, t);
                count += 1;
                if count >= 3 {
                    // empty the queue
                    while !queue.is_empty() {
                        match thread_worker.send(queue.pop_front().unwrap()) {
                            Ok(_) => {}
                            Err(e) => {println!("Error sending channel data: {:?}", e)}
                        };
                    }
                }
            }
        }
    });
}

fn freceive(rx: Receiver<ChannelData>, num_tasks: &i32, num_agents: &i32) {
    for t in (0..*num_tasks).rev() {
        for a in (0..*num_agents).rev() {
            // receive a matrix
            match rx.recv() {
                Ok(mut m) => {
                    //println!("data: {:?}", m.A.x);
                    //println!("i: {:?}", m.A.i);
                    //println!("j: {:?}", m.A.p);
                    let T = create_sparse_matrix(
                        m.S.m,
                        m.S.n,
                        &mut m.S.i[..],
                        &mut m.S.j[..],
                        &mut m.S.x[..]
                    );
                    let _A = convert_to_compressed(T);
                    //print_matrix(A);
                    //println!("received mat_{}_{}", m.a, m.t);
                    assert_eq!(m.t, t, "task index not equal to expected");
                    assert_eq!(m.a, a, "agent index not equal to expected");
                    // some long running task
                    //thread::sleep(time::Duration::from_secs(10));
                    // throw it away: manage memory
                    //free_sparse(A);
                }
                Err(_) => {
                    println!("received an error!")
                }
            };
        }
    }
}

fn create_mat_files(num_tasks: &i32, num_agents: &i32) {
    let msize = 50000;
    let nsize = 50000;
    //let j_ind = (0..nsize).collect::<Vec<i32>>();
    let agents = *num_agents;
    let tasks = *num_tasks;
    // store some matrices as yaml files
    // create a channel
    // send the files to a sync_channel which is bounded
    // todo this is actually too complicated. What we require is that a threadpool takes
    //  care of constructing a sparse matrix and saving it to disk concurrently
    // sender
    let t1 = Instant::now();
    let pool = ThreadPool::new(20);
    for t in 0..tasks {
        for a in 0..agents {
            pool.execute(move || {
                let mut rng = thread_rng();
                let mut i: Vec<i32> = Vec::new();
                let mut j: Vec<i32> = Vec::new();
                let mut vals: Vec<f64> = Vec::new();
                for ii in 0..msize {
                    for jj in (0..nsize).choose_multiple(&mut rng, 2) {
                        i.push(ii);
                        j.push(jj);
                        vals.push(rng.gen());
                    }
                }
                let nnz = vals.len() as i32;
                assert_eq!(i.len(), j.len(), "index lengths don't match");
                let T = COO {
                    nzmax: nnz,
                    m: msize,
                    n: nsize,
                    i,
                    j,
                    x: vals,
                    nz: nnz
                };
                let filename = format!("mat_{}_{}.yml", a, t);
                T.store_matrix_as_yaml(filename.as_str());
            });
        }
    }
    pool.join();
    println!("{:?}", t1.elapsed().as_secs_f64());
}

struct ChannelData {
    S: COO,
    a: i32,
    t: i32
}