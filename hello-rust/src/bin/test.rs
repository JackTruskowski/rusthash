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

// mod hash_table;

mod hash_table_restriction1;
// use crate::hash_table;
extern crate atomic_traits;
extern crate atomic_refcell;
extern crate fasthash;
extern crate crossbeam;

//const HT_SIZE: u32 = 1048576; //must be power of 2
const LOAD_FACTOR: f32 = 0.80;
//const NUM_OPS: i32 = 1000000;
const HT_SIZE: u32 = 67108864; //must be power of 2
const NUM_OPS: i32 = 25000000;

//
//@param Hash table
//@param Number of inserts for each thread
//@param Number of threads
fn insert_and_find(ht: hash_table_restriction1::HashTable, stored_keys: Vec<(u32, u32)>, num_threads: i32) -> (f64, f64) {

    let ht = Arc::new(ht);
    let mut handles = vec![];
    let adds_per_thread = (stored_keys.len() as i32) / num_threads;
    let total_adds = stored_keys.len();

    let start = Instant::now();
    for i in 0..num_threads {

        let ht = Arc::clone(&ht);

        let handle = thread::spawn(move || {

	    for j in 0..adds_per_thread {

		//let mut val = s.pop();
		//randomly generate and add a (key, value)
		let key = thread_rng().gen::<u32>();
		let value = thread_rng().gen::<u32>();
		// match val {
		//     Some(x) => {
		// 	ht.set_item(x.0, x.1);
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
    //  	println!("{}", stored_keys.lock().unwrap()[idx]);
    // }

    let elapsed_time = start.elapsed();

    // println!("----------------------------------------");
    // println!("Number of Threads: {}", num_threads);
    // println!("Total number of insertions: {:?}", (num_threads * (adds_per_thread as i32)));
    // println!("Total time: {:?}", elapsed_time.as_millis());
    // println!("Throughput: {:.3} ops/ms", (total_adds as f64 / elapsed_time.as_millis() as f64));


    let in_thr = (total_adds as f64 / (1000000 as f64) / elapsed_time.as_secs_f64()); //insert throughput


    let mut handles = vec![];
    let start = Instant::now();

    for i in 0..num_threads {

	let ht = Arc::clone(&ht);

        let handle = thread::spawn(move || {

	    for j in 0..adds_per_thread {
		// let mut value : u32 = 0;
		// let kv_pair = s.pop();

		let key = thread_rng().gen::<u32>();
		let value = thread_rng().gen::<u32>();

		// match kv_pair {
		//     Some(x) => {
		// 	value = ht.get_item(x.0);
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

    let get_thr = (total_adds as f64 / (1000000 as f64) / elapsed_time.as_secs_f64()); //insert throughput


    (in_thr, get_thr)
}


fn run_benchmark(ht_size: u32, load_factor: f32, inserts: Vec<(u32, u32)>, num_threads: i32) -> (f64, f64) {
    let ht = hash_table_restriction1::HashTable::new(ht_size, load_factor);
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


    throughputs.push(run_benchmark(HT_SIZE, LOAD_FACTOR, inserts.clone(), 1));
    throughputs.push(run_benchmark(HT_SIZE, LOAD_FACTOR, inserts.clone(), 2));
    throughputs.push(run_benchmark(HT_SIZE, LOAD_FACTOR, inserts.clone(), 4));
    throughputs.push(run_benchmark(HT_SIZE, LOAD_FACTOR, inserts.clone(), 8));
    throughputs.push(run_benchmark(HT_SIZE, LOAD_FACTOR, inserts.clone(), 12));
    throughputs.push(run_benchmark(HT_SIZE, LOAD_FACTOR, inserts.clone(), 16));
    throughputs.push(run_benchmark(HT_SIZE, LOAD_FACTOR, inserts.clone(), 32));
    throughputs.push(run_benchmark(HT_SIZE, LOAD_FACTOR, inserts.clone(), 48));

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















// // use hash_table;
// mod hash_table_restriction1;
// // use crate::hash_table;
// extern crate atomic_traits;
// extern crate atomic_refcell;
// extern crate fasthash;
// extern crate crossbeam;
// use std::sync::atomic::AtomicU32;
// use std::sync::atomic::AtomicU64;
// use std::sync::atomic::Ordering;
// use std::u32;


// fn main() {

// 	// test0();
// 	test1();
// 	// test2();
// }


// fn test0() {

// 	println!("test0");
// 	let a  = AtomicU32::new(u32::MAX);
// 	let mut res = a.compare_and_swap(0, 10, Ordering::Relaxed);

// 	if res == u32::MAX {
// 		res = a.compare_and_swap(u32::MAX, 10, Ordering::Relaxed);
// 	}

// 	println!("returned:{}, saved:{}", res, a.load(Ordering::Relaxed));
// }


// fn test2() {
// 	let a = 12;
// 	println!("Power of 2 for {} is {}", a, next_power_of_2_double_the(a));
// }


// fn next_power_of_2_double_the(num: u32) -> u32 {
//     	let mut v = num*2;

//     	v -= 1;
//     	v |= v >> 1;
//     	v |= v >> 2;
//     	v |= v >> 4;
//     	v |= v >> 8;
//     	v |= v >> 16;
//     	v += 1;
//     	v
//     }


// fn test1() {

// 	println!("test1");

// 	let mut ht: hash_table_restriction1::HashTable = hash_table_restriction1::HashTable::new(8, 0.5);
// 	ht.print_ht_contents();
// 	println!("Inserting {}", 1);
// 	ht.set_item(1, 9);
// 	println!("Get item count = {}", ht.get_item_count());
// 	println!("Inserting {}", 10);
// 	ht.set_item(10, 19);
// 	println!("Inserting {}", 32);
// 	ht.set_item(32, 90);
// 	println!("Inserting {}", 23);
// 	ht.set_item(23, 91);
// 	println!("Get item count = {}", ht.get_item_count());
// 	// println!("Hash = {}", hash_table_restriction1::hash1(33));
// 	println!();
// 	ht.print_ht_contents();
// 	println!();

// 	// ht.manipulate_vec();


// 	println!("Value for key:{} is {}", 10, ht.get_item(10));
// 	println!("Remove key:{}", 10);
// 	ht.print_ht_contents();
// 	println!();
// 	println!("Get item count = {}", ht.get_item_count());

// 	println!("Inserting {}", 10);
// 	ht.set_item(10, 190);
// 	ht.print_ht_contents();
// 	println!();
// 	println!("Get item count = {}", ht.get_item_count());

// 	println!("Inserting {}", 15);
// 	ht.set_item(15, 190);
// 	ht.print_ht_contents();
// 	println!();
// 	println!("Get item count = {}", ht.get_item_count());

// 	println!("Value for key:{} is {}", 15, ht.get_item(15));
// 	println!("Remove key:{}", 15);
// 	println!("Value for key:{} is {}", 15, ht.remove_item(15));
// 	ht.print_ht_contents();
// 	println!();
// 	println!("Value for key:{} is {}", 15, ht.get_item(15));
// 	println!("Get item count = {}", ht.get_item_count());


// 	println!("Inserting {}", 12);
// 	ht.set_item(12, 19);
// 	println!("Inserting {}", 13);
// 	ht.set_item(13, 90);
// 	println!("Inserting {}", 14);
// 	ht.set_item(14, 91);
// 	println!("Inserting {}", 16);
// 	ht.set_item(16, 19);
// 	println!("Inserting {}", 17);
// 	ht.set_item(17, 90);
// 	println!("Inserting {}", 18);
// 	ht.set_item(18, 91);
// 	println!("Get item count = {}", ht.get_item_count());




// 	println!("Value for key:{} is {}", 15, ht.get_item(15));
// 	println!("Remove key:{}", 15);
// 	println!("Value for key:{} is {}", 15, ht.remove_item(15));
// 	ht.print_ht_contents();
// 	println!();
// 	println!("Value for key:{} is {}", 15, ht.get_item(15));
// 	println!("Get item count = {}", ht.get_item_count());

// 	ht.set_item(15, 1);
// 	ht.print_ht_contents();
// 	println!();
// 	println!("Get item count = {}", ht.get_item_count());

// 	println!("Remove key:{}", 10);
// 	println!("Value for key:{} is {}", 10, ht.remove_item(10));
// 	ht.print_ht_contents();
// 	println!();
// 	println!("Get item count = {}", ht.get_item_count());

// 	println!("Inserting {}", 10);
// 	ht.set_item(10, 190);
// 	ht.print_ht_contents();
// 	println!();
// 	println!("Get item count = {}", ht.get_item_count());


// 	println!("Remove key:{}", 11);
// 	println!("Value for key:{} is {}", 11, ht.remove_item(11));
// 	println!("Get item count = {}", ht.get_item_count());

// 	// println!("Array size = {}, Item count = {}", ht.get_array_size(), ht.get_item_count());
// 	// println!("{}, {}, {}, {}", ht.get_item(1), ht.get_item(10), ht.get_item(32), ht.get_item(23));
// 	// println!("Array size = {}, Item count = {}", ht.get_array_size(), ht.get_item_count());
// 	// ht.clear();
// 	// println!("Array size = {}, Item count = {}", ht.get_array_size(), ht.get_item_count());
// 	// ht.print_ht_contents();
// 	println!();

// }
