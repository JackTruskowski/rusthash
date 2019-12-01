use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use core::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use rand::prelude::*;

mod hash_table;

fn main() {

    //hyperparameters
    let HT_SIZE = 1024;
    let ADDS_PER_THREAD = 10;
    let NUM_THREADS = 20;

    let ht = Arc::new(hash_table::HashTable::new(HT_SIZE));
    let mut handles = vec![];
    let mut stored_keys: Vec<u32> = Vec::new();
    let total_duration = Arc::new(Mutex::new(Duration::new(0,0)));


    for i in 0..NUM_THREADS {
	println!("Thread {} starting", i);
        let ht = Arc::clone(&ht);
	let total_duration = Arc::clone(&total_duration);

        let handle = thread::spawn(move || {

	    let mut total_time = Duration::new(0,0);

	    for _ in 0..ADDS_PER_THREAD {
		let key = thread_rng().gen::<u32>();
		let value = thread_rng().gen::<u32>();
		stored_keys.push(key);

		let start = Instant::now();
		ht.set_item(key, value);

		let elapsed_time = start.elapsed();
		total_time += elapsed_time;


		println!("\tThread {}: [{}, {}] in {:?}", i, key, value, elapsed_time);
	    }

	    //update time using a Mutex
	    {
		let mut num = total_duration.lock().unwrap();
		*num += total_time;

	    }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let float_millis = total_duration.lock().unwrap().as_nanos() as f64;
    let total_adds = (NUM_THREADS * ADDS_PER_THREAD) as f64;


    println!("----------------------------------------");
    println!("Total number of insertions: {:?}", (NUM_THREADS * ADDS_PER_THREAD));
    println!("Total wall-time of insertions: {:?}", total_duration.lock().unwrap());
    println!("Average wall-time of insertions: {}ns", (float_millis / total_adds));
}
