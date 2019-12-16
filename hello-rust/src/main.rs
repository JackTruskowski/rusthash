use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use rand::prelude::*;
use rand::Rng;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;
use std::collections::HashMap;

mod hash_table;

//const HT_SIZE: u32 = 67108864; //must be power of 2
//const NUM_OPS: i32 = 25000000;
const HT_SIZE: u32 = 16384; //must be power of 2
const NUM_OPS: i32 = 10000;
type KeySize = u32;
type ValSize = u64;


fn basic_hm(inserts: Vec<(KeySize,ValSize)>) -> (f64, f64){
    let mut my_hm = HashMap::new();
    let mut insert_keys = inserts.clone();
    let start = Instant::now();
    for i in 0..NUM_OPS {
        let tmp = insert_keys.pop();
        match tmp {
            Some(x) => {
                my_hm.insert(x.0, x.1);
            }
            None => println!("Error!"),
        }
    }
    let elapsed_time = start.elapsed();

    let insert_thr = NUM_OPS as f64 / (1000000 as f64) / elapsed_time.as_secs_f64();

    let start = Instant::now();
    let mut find_keys = inserts.clone();

    for i in 0..NUM_OPS {
        let tmp = find_keys.pop();
        match tmp {
            Some(x) => {
                my_hm[&x.0];
            }
            None => println!("Error!"),
        }
    }
    let elapsed_time = start.elapsed();
    let find_thr = NUM_OPS as f64 / (1000000 as f64) / elapsed_time.as_secs_f64();

    (insert_thr, find_thr)
}


//
//@param Hash table
//@param Number of inserts for each thread
//@param Number of threads
fn insert_and_find_32(ht: hash_table::HashTable, total_adds: i32, num_threads: i32) -> (f64, f64) {

    let ht = Arc::new(ht);
    let mut handles = vec![];
    let adds_per_thread = total_adds / num_threads;

    let start = Instant::now();

    for i in 0..num_threads {

        let ht = Arc::clone(&ht);

        let handle = thread::spawn(move || {

            for j in 0..adds_per_thread {

                //let mut val = s.pop();
                //randomly generate and add a (key, value)
                let key = thread_rng().gen::<KeySize>();
                let value = thread_rng().gen::<ValSize>();
                // match val {
                //     Some(x) => {
                //      ht.set_item(x.0, x.1);
                //     }
                //     None => println!("Problem popping a stored key"),
                // }
                ht.set_item(key, value);

            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // let len = stored_keys.lock().unwrap().len();
    // for idx in 0..len {
    //          println!("{}", stored_keys.lock().unwrap()[idx]);
    // }

    let elapsed_time = start.elapsed();
    // println!("Elapsed time (sec) = {}", elapsed_time.as_secs_f64());
    // println!("Total adds = {}", total_adds);
    // println!("MOps (total adds / 1,000,000) = {}", total_adds as f64 / 1000000.0);
    let in_thr = (total_adds as f64 / (1000000 as f64) / elapsed_time.as_secs_f64()); //insert throughput


    let mut handles = vec![];
    let start = Instant::now();
    //let stored_keys_lock = Arc::new(Mutex::new(stored_keys.clone()));

    let start = Instant::now();


    for i in 0..num_threads {

        let ht = Arc::clone(&ht);


        let handle = thread::spawn(move || {

            for j in 0..adds_per_thread {

                // let mut value : u32 = 0;
                // let kv_pair = s.pop();

                let key = thread_rng().gen::<KeySize>();
                let value = thread_rng().gen::<ValSize>();

                // match kv_pair {
                //     Some(x) => {
                //      value = ht.get_item(x.0);
                //     },
                //     None => {},
                // }
                let _ = ht.get_item(key);

            }

        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


    let elapsed_time = start.elapsed();

    let get_thr = (total_adds as f64 / (1000000 as f64) / elapsed_time.as_secs_f64()); //find throughput

    //(total_time, get_thr)
    (in_thr, get_thr)
}

fn run_benchmark(ht_size: u32, num_inserts: i32, num_threads: i32) -> (f64, f64) {
    let ht = hash_table::HashTable::new(ht_size);
    insert_and_find_32(ht, num_inserts, num_threads)
}

fn print_speedup(thrputs: Vec<(f64, f64)>) {
    assert!(thrputs.len() > 1);

    for i in 0..thrputs.len() {
        if i == 0 {
            continue;
        }
        println!("{}\t{}", thrputs[i].0 / thrputs[0].0, thrputs[i].1 / thrputs[0].1);
    }
}

fn main() {

    let mut throughputs = Vec::new();

    //values to be inserted
    let mut inserts = Vec::new();
    for i in 0..NUM_OPS {

        //randomly generate and add a (key, value)
        let key = thread_rng().gen::<KeySize>();
        let value = thread_rng().gen::<ValSize>();

        inserts.push((key, value));
    }

    println!("Running the built-in Rust hashmap...");
    let res = basic_hm(inserts.clone());
    throughputs.push(res);

    println!("Benchmarking the concurrent implementation...");
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 1));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 2));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 4));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 8));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 12));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 16));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 32));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 48));
    print_speedup(throughputs.clone());

    //Write to csv file
    if let Err(err) = run(throughputs) {
        println!("{}", err);
        process::exit(1);
    }
}

//modified from rust docs
fn run(arr: Vec<(f64, f64)>) -> Result<(), Box<Error>> {

    let file_path = get_first_arg()?;
    let mut wtr = csv::Writer::from_path(file_path)?;

    let mut insert_records = Vec::new();
    let mut find_records = Vec::new();

    for i in 0..arr.len() {
        let current_thr = arr[i];
        insert_records.push(current_thr.0.to_string());
        find_records.push(current_thr.1.to_string());
    }


    wtr.write_record(insert_records)?;
    wtr.write_record(find_records)?;

    wtr.flush()?;
    Ok(())
}

//from rust docs
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
