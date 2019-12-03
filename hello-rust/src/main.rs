use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use rand::prelude::*;

mod hash_table;

//
//@param Hash table
//@param Number of inserts for each thread
//@param Number of threads
fn insert(ht: hash_table::HashTable, adds_per_thread: i32, num_threads: i32) -> Vec<u32> {


    let ht = Arc::new(ht);
    let mut handles = vec![];

    let stored_keys = Arc::new(Mutex::new(Vec::new()));
    let total_duration = Arc::new(Mutex::new(Duration::new(0,0)));


    for i in 0..num_threads {
	println!("Thread {} starting", i);

	//clone shared data structures so they're visible
	//to all threads
	let sk = Arc::clone(&stored_keys);
        let ht = Arc::clone(&ht);
	let total_duration = Arc::clone(&total_duration);

        let handle = thread::spawn(move || {

	    let mut total_time = Duration::new(0,0);

	    for _ in 0..adds_per_thread {

		//randomly generate and add a (key, value)
		let key = thread_rng().gen::<u32>();
		let value = thread_rng().gen::<u32>();
		{
		    let mut s = sk.lock().unwrap();
		    s.push(key);
		}

		//measure the time to add
		let start = Instant::now();
		ht.set_item(key, value);
		let elapsed_time = start.elapsed();

		total_time += elapsed_time;


		//println!("\tThread {}: [{}, {}] in {:?}", i, key, value, elapsed_time);
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

    // let len = stored_keys.lock().unwrap().len();
    // for idx in 0..len {
    //  	println!("{}", stored_keys.lock().unwrap()[idx]);
    // }

    let float_millis = total_duration.lock().unwrap().as_nanos() as f64;
    let total_adds = (num_threads * adds_per_thread) as f64;

    println!("----------------------------------------");
    println!("Number of Threads: {}", num_threads);
    println!("Total number of insertions: {:?}", (num_threads * adds_per_thread));
    println!("Total wall-time of insertions: {:?}", total_duration.lock().unwrap());
    println!("Average wall-time of insertions: {}ns", (float_millis / total_adds));

    //return a vector of the keys we just inserted
    //TODO: fix
    //(*Arc::make_mut(&mut stored_keys)).get_mut().unwrap()
    //&*stored_keys.get_mut().unwrap()
}


fn main() {

    let ht = hash_table::HashTable::new(1024);
    insert(ht, 150, 5);

}
