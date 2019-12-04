use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use rand::prelude::*;
use rand::Rng;


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

		// if j % 100000 == 0 {
		//     println!("Thread {} inserted {}", i, j);
		// }

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

    // let float_millis = total_duration.lock().unwrap().as_nanos() as f64;
    // let total_adds = (num_threads * adds_per_thread) as f64;

    println!("----------------------------------------");
    println!("Number of Threads: {}", num_threads);
    println!("Total number of insertions: {:?}", (num_threads * adds_per_thread));
    println!("Total time: {:?}", elapsed_time.as_millis());
    println!("Throughput: {:.3} ops/ms", (total_adds as f64 / elapsed_time.as_millis() as f64));
    //println!("Total wall-time of insertions: {:?}", total_duration.lock().unwrap());
    //println!("Average wall-time of insertions: {}ns", (float_millis / total_adds));

    //return a vector of the keys we just inserted
    //TODO: fix
    //(*Arc::make_mut(&mut stored_keys)).get_mut().unwrap()
    //&*stored_keys.get_mut().unwrap()
    (total_adds as f64 / (1000000 as f64) / elapsed_time.as_millis() as f64) //throughput

}


fn main() {

    let ht = hash_table::HashTable::new(1048576);
    let single_thr = insert(ht, 1000000, 1);

    let ht2 = hash_table::HashTable::new(1048576);
    let two_thr = insert(ht2, 1000000, 2);
    println!("1 -> 2 speedup = {}", two_thr/single_thr);

    let ht4 = hash_table::HashTable::new(1048576);
    let four_thr = insert(ht4, 1000000, 4);
    println!("1 -> 4 speedup = {}", four_thr/single_thr);

    let ht8 = hash_table::HashTable::new(1048576);
    let eight_thr = insert(ht8, 1000000, 8);
    println!("1 -> 8 speedup = {}", eight_thr/single_thr);

    let ht12 = hash_table::HashTable::new(1048576);
    let tw_thr = insert(ht12, 1000000, 12);
    println!("1 -> 12 speedup = {}", tw_thr/single_thr);

}
