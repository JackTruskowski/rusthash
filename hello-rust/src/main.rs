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


//
//@param Hash table
//@param Number of inserts for each thread
//@param Number of threads
fn insert(ht: hash_table::HashTable, total_adds: i32, num_threads: i32) -> f64 {

    let ht = Arc::new(ht);
    let mut handles = vec![];

    // let stored_keys = Arc::new(Mutex::new(Vec::new()));
    // let total_duration = Arc::new(Mutex::new(Duration::new(0,0)));
    let adds_per_thread = total_adds / num_threads;

    let start = Instant::now();
    for i in 0..num_threads {
	//println!("Thread {} starting", i);

	//clone shared data structures so they're visible
	//to all threads
	//let sk = Arc::clone(&stored_keys);
        let ht = Arc::clone(&ht);
	//let total_duration = Arc::clone(&total_duration);

        let handle = thread::spawn(move || {

	    //let mut total_time = Duration::new(0,0);

	    for j in 0..adds_per_thread {

		//randomly generate and add a (key, value)
		let key = thread_rng().gen::<u32>();
		let value = thread_rng().gen::<u32>();
		// {
		//     let mut s = sk.lock().unwrap();
		//     s.push(key);
		// }

		//measure the time to add
		//let start = Instant::now();
		ht.set_item(key, value);
		//let elapsed_time = start.elapsed();

		//total_time += elapsed_time;


		//println!("\tThread {}: [{}, {}] in {:?}", i, key, value, elapsed_time);
	    }

	    //update time using a Mutex
	    //{
	    //let mut num = total_duration.lock().unwrap();
	    //*num += total_time;
	    //}
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


fn main() {

    let ht = hash_table::HashTable::new(1048576);
    let single_thr = insert(ht, 1000000, 1);

    let mut array: [f64; 5] = [0.0; 5];

    //2 Threads
    let ht2 = hash_table::HashTable::new(1048576);
    let two_thr = insert(ht2, 1000000, 2);
    array[0] = two_thr/single_thr;
    println!("1 -> 2 speedup = {}", two_thr/single_thr);

    //4 Threads
    let ht4 = hash_table::HashTable::new(1048576);
    let four_thr = insert(ht4, 1000000, 4);
    array[1] = four_thr/single_thr;
    println!("1 -> 4 speedup = {}", four_thr/single_thr);

    //8 Threads
    let ht8 = hash_table::HashTable::new(1048576);
    let eight_thr = insert(ht8, 1000000, 8);
    array[2] = eight_thr/single_thr;
    println!("1 -> 8 speedup = {}", eight_thr/single_thr);

    //12 Threads
    let ht12 = hash_table::HashTable::new(1048576);
    let tw_thr = insert(ht12, 1000000, 12);
    array[3] = tw_thr/single_thr;
    println!("1 -> 12 speedup = {}", tw_thr/single_thr);

    //24 Threads
    let ht24 = hash_table::HashTable::new(1048576);
    let tf_thr = insert(ht24, 1000000, 24);
    array[4] = tf_thr/single_thr;
    println!("1 -> 12 speedup = {}", tf_thr/single_thr);


    //Write to csv file
    if let Err(err) = run(&mut array) {
        println!("{}", err);
        process::exit(1);
    }
}

//modified from rust docs
fn run(arr: &mut [f64]) -> Result<(), Box<Error>> {

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
