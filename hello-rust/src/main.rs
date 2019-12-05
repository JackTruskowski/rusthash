use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
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
fn insert_and_find(ht: hash_table::HashTable, total_adds: i32, num_threads: i32) -> f64 {

    let ht = Arc::new(ht);
    let mut handles = vec![];

    let stored_keys = Arc::new(Mutex::new(Vec::new()));
    let adds_per_thread = total_adds / num_threads;

    //TODO: using a mutex while measuring time is probably not good
    //but might not matter if our metric is speedup rather than throughput
    let start = Instant::now();
    for i in 0..num_threads {

        let ht = Arc::clone(&ht);
	let sk = Arc::clone(&stored_keys);

        let handle = thread::spawn(move || {

	    for j in 0..adds_per_thread {

		//randomly generate and add a (key, value)
		let key = thread_rng().gen::<u32>();
		let value = thread_rng().gen::<u32>();
		{
		    let mut s = sk.lock().unwrap();
		    s.push(key);
		}

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
    //  	println!("{}", stored_keys.lock().unwrap()[idx]);
    // }

    let elapsed_time = start.elapsed();

    println!("----------------------------------------");
    println!("Number of Threads: {}", num_threads);
    println!("Total number of insertions: {:?}", (num_threads * adds_per_thread));
    println!("Total time: {:?}", elapsed_time.as_millis());
    println!("Throughput: {:.3} ops/ms", (total_adds as f64 / elapsed_time.as_millis() as f64));


    (total_adds as f64 / (1000000 as f64) / elapsed_time.as_millis() as f64) //throughput

}


fn run_benchmark(ht_size: u32, num_ops: i32, num_threads: i32) -> f64 {
    let ht = hash_table::HashTable::new(ht_size);
    insert_and_find(ht, num_ops, num_threads)
}

fn print_speedup(thrputs: Vec<f64>) {
    assert!(thrputs.len() > 1);

    for i in 0..thrputs.len() {
	if i == 0 {
	    continue;
	}
	println!("{}", thrputs[i] / thrputs[0]);
    }
}

fn main() {

    let mut throughputs = Vec::new();
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 1));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 2));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 4));
    throughputs.push(run_benchmark(HT_SIZE, NUM_OPS, 8));

    print_speedup(throughputs.clone());

    //Write to csv file
    if let Err(err) = run(throughputs) {
        println!("{}", err);
        process::exit(1);
    }
}

//modified from rust docs
fn run(arr: Vec<f64>) -> Result<(), Box<Error>> {

    let file_path = get_first_arg()?;
    let mut wtr = csv::Writer::from_path(file_path)?;

    wtr.write_record(&[arr[0].to_string(), arr[1].to_string(), arr[2].to_string(), arr[3].to_string(), arr[4].to_string()])?;

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
