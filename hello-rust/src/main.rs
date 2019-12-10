use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::sync::atomic::AtomicU32;
use std::thread;
use rand::prelude::*;
use rand::Rng;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

mod hash_table;

const HT_SIZE: u32 = 1048576; //must be power of 2
const NUM_OPS: i32 = 1000000;


//
//@param Hash table
//@param Number of inserts for each thread
//@param Number of threads
fn insert_and_find(ht: hash_table::HashTable, stored_keys: Vec<(u32, u32)>, num_threads: i32) -> (f64, f64) {

    let ht = Arc::new(ht);
    let mut handles = vec![];
    let adds_per_thread = (stored_keys.len() as i32) / num_threads;
    let total_adds = stored_keys.len();
    //let stored_keys_lock = Arc::new(Mutex::new(stored_keys.clone()));

    let start = Instant::now();
    for i in 0..num_threads {

        let ht = Arc::clone(&ht);
	let mut s = stored_keys.clone();

        let handle = thread::spawn(move || {

	    for j in 0..adds_per_thread {

		let mut val = s.pop();
		//println!("{:?}", val);

		match val {
		    Some(x) => ht.set_item(x.0, x.1),
		    None => println!("Problem popping a stored key"),
		}

	    }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // let len = stored_keys.lock().unwrap().len();
    // for idx in 0..len {
    //  	println!("{}", stored_keys.lock().unwrap()[idx]);
    // }

    let elapsed_time = start.elapsed();
    let in_thr = (total_adds as f64 / (1000000 as f64) / elapsed_time.as_millis() as f64); //insert throughput


    let mut handles = vec![];
    let start = Instant::now();
    //let stored_keys_lock = Arc::new(Mutex::new(stored_keys.clone()));
    for i in 0..num_threads {

	let ht = Arc::clone(&ht);
	let mut s = stored_keys.clone();

        let handle = thread::spawn(move || {

	    for j in 0..adds_per_thread {

		let mut value : u32 = 0;
		let mut kv_pair = s.pop();
		match kv_pair {
		    Some(x) => {
			value = ht.get_item(x.0);
		    },
		    None => {},
		}
	    }

        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_time = start.elapsed();

    let get_thr = (total_adds as f64 / (1000000 as f64) / elapsed_time.as_millis() as f64); //insert throughput


    (in_thr, get_thr)
}


fn run_benchmark(ht_size: u32, inserts: Vec<(u32, u32)>, num_threads: i32) -> (f64, f64) {
    let ht = hash_table::HashTable::new(ht_size);
    insert_and_find(ht, inserts, num_threads)
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
	let key = thread_rng().gen::<u32>();
	let value = thread_rng().gen::<u32>();

	inserts.push((key, value));
    }


    throughputs.push(run_benchmark(HT_SIZE, inserts.clone(), 1));
    throughputs.push(run_benchmark(HT_SIZE, inserts.clone(), 4));
    throughputs.push(run_benchmark(HT_SIZE, inserts.clone(), 8));
    throughputs.push(run_benchmark(HT_SIZE, inserts.clone(), 12));
    throughputs.push(run_benchmark(HT_SIZE, inserts.clone(), 16));
    throughputs.push(run_benchmark(HT_SIZE, inserts.clone(), 24));

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

    let single_speedup = arr[0];
    insert_records.push(1.to_string());
    find_records.push(1.to_string());
    for i in 1..arr.len() {
	let current_speedup = arr[i];
	insert_records.push((current_speedup.0/single_speedup.0).to_string());
	find_records.push((current_speedup.1/single_speedup.1).to_string());
    }
    ;

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
