#[macro_use] extern crate log;
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use rand::prelude::*;

mod hash_table;

fn main() {

    let ht = Arc::new(hash_table::HashTable::new(64));
    let mut handles = vec![];

    for i in 0..4 {
	println!("Thread {} starting", i);
        let ht = Arc::clone(&ht);
        let handle = thread::spawn(move || {

	    for _ in 0..10 {
		let key = thread_rng().gen::<u32>();
		let value = thread_rng().gen::<u32>();

		ht.set_item(key, value);
		println!("\tThread {}: [{}, {}]", i, key, value);
	    }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Map contents: ");
    ht.print_ht_contents();

}
