// This Hash table add remove and resize functionality to the jeff preshing code



use std::sync::atomic::AtomicU32;
use std::sync::atomic::AtomicU64;
use crossbeam::atomic::AtomicCell;
use crossbeam::atomic::AtomicConsume;
use atomic_refcell::AtomicRef;
use std::sync::{Arc, Mutex};
use std::sync::RwLock;
use std::sync::atomic::Ordering;
use std::convert::TryInto;
// use std::hash::{Hash, Hasher};
// use std::collections::hash_map::DefaultHasher;
// use atomic_traits::{Atomic};
use std::u32;
use std::mem;
use std::cell::Cell;
use std::rc::Rc;
// use std::clone::Clone;
use std::marker::Copy;
use std::default::Default;
// use std::hash::{Hash, Hasher};

// use fasthash::{murmur3, Murmur3Hasher};


// fn hash<T: Hash>(t: &T) -> u64 {
//     let mut s: Murmur3Hasher = Default::default();
//     t.hash(&mut s);
//     s.finish()
// }




// trait Hash {
// 	fn hash(&self) -> u32;
// 	fn hash(&self) -> u64;
// }

// impl Hash for u32 {
// 	fn hash<H>(&self, state: &mut H) where H: Hasher {
// 		self.hash(state)
// 	}
// }


// // impl Hash for u32 {
// impl Hash for AtomicU32 {
// 	fn hash(&self) -> u32 {
// 		let mut h = self.load(Ordering::Relaxed);
// 		h ^= h >> 16;
// 		//wrapping_mul function achieves desired C++
// 		//integer overflow wraparound behavior
// 		h = h.wrapping_mul(0x85ebca6b);
// 	    	h ^= h >> 13;
// 		h = h.wrapping_mul(0xc2b2ae35u32);
// 	    	h ^= h >> 16;
// 		h
// 	}
// }


// // impl Hash for u64 {
// impl Hash for AtomicU64 {
// 	fn hash(&self) -> u64 {
// 		let mut h = self.load(Ordering::Relaxed);
// 		h ^= h >> 33;
// 		//wrapping_mul function achieves desired C++
// 		//integer overflow wraparound behavior
// 		h = h.wrapping_mul(0xff51afd7ed558ccd);
// 	    	h ^= h >> 33;
// 		h = h.wrapping_mul(0xc4ceb9fe1a85ec53u64);
// 	    	h ^= h >> 33;
// 		h
// 	}
// }

// pub fn hash<H>(&self, state: &mut H) where H: Hasher {
// 		self.hash(state)
// }

const TOMBSTONE:u32 = u32::MAX;


//Rust port of Jeff Preshing's simple lock-free hash table
// #[derive(Copy)]
#[derive(Default)]
pub struct Entry {
    key: AtomicU32,
    value: AtomicU32,
}


impl Clone for Entry {
    fn clone(&self) -> Self {
        Entry {
            key: AtomicU32::new(self.key.load(Ordering::SeqCst)),
            value: AtomicU32::new(self.value.load(Ordering::SeqCst))
        }
    }
}



impl Entry {
    pub fn new() -> Self{
		Self {
		    key: AtomicU32::new(0),
		    value: AtomicU32::new(0)
		}
    }
}


pub struct HashTable<> {
    //size must be known at compile-time for rust arrays
    //Vectors appear to be how to do Java-style arrays
    // m_entries: <Vec<Entry>>,
    m_entries: Arc<RwLock<Vec<Entry>>>,
    m_array_size: AtomicCell<u32>,
    load_factor_thres: f32,
    item_count: AtomicCell<u32>,
}

impl HashTable {

    //constructor
    pub fn new(max_size: u32, load_factor: f32) -> Self {
		assert!((max_size & (max_size -1)) == 0);

		let mut my_vec: Vec<Entry> = Vec::new();
		for _ in 0..max_size {
		    my_vec.push(Entry::new());
		}

		Self {
		    m_entries: Arc::new(RwLock::new(my_vec)),
		    m_array_size: AtomicCell::new(max_size),
		    load_factor_thres: load_factor,
		    item_count: AtomicCell::new(0)
		}
    }


    


    //from code.google.com/p/smhasher/wiki/MurmurHash3
    fn integer_hash(mut h: u32) -> u32 {
    	h ^= h >> 16;
		//wrapping_mul function achieves desired C++
		//integer overflow wraparound behavior
		h = h.wrapping_mul(0x85ebca6b);
	    	h ^= h >> 13;
		h = h.wrapping_mul(0xc2b2ae35u32);
	    	h ^= h >> 16;
		h
    }


  //   //from code.google.com/p/smhasher/wiki/MurmurHash3
  //   fn integer_hash64(mut h: u64) -> u64 {
  //   	h ^= h >> 33;
	 //    h = h.wrapping_mul(0xff51afd7ed558ccd);
	 //    h ^= h >> 33;
	 //    h = h.wrapping_mul(0xc4ceb9fe1a85ec53u64);
	 //    h ^= h >> 33;
		// h
  //   }


    pub fn set_item(& self, key:u32, value:u32) {

		//0 reserved for 'empty' value
		assert!(key != 0 && key != TOMBSTONE);
		assert!(value != 0);

		let mut idx = HashTable::integer_hash(key);
		// let mut idx = murmur3::hash(key);
		// let mut idx = hash(&key);
		loop {

		    //scale to size of array
		    idx &= self.m_array_size.load() - 1;

		    let lockedr_entries = self.m_entries.read().unwrap();
		    // let mut result_key = self.m_entries[HashTable::u32_to_usize(idx)].key.compare_and_swap(0, key, Ordering::Relaxed);
		    let mut result_key = lockedr_entries[HashTable::u32_to_usize(idx)].key.compare_and_swap(0, key, Ordering::Relaxed);

		    if result_key == TOMBSTONE {
				result_key = lockedr_entries[HashTable::u32_to_usize(idx)].key.compare_and_swap(TOMBSTONE, key, Ordering::Relaxed);		    	
		    }

		    if result_key == 0 || result_key == key || result_key == TOMBSTONE {
				lockedr_entries[HashTable::u32_to_usize(idx)].value.store(value, Ordering::Relaxed);
				HashTable::log_message(format!("added value {} at index {}", value, idx), 2);
				self.item_count.swap(self.item_count.load()+1);
				break;
		    }

		    HashTable::log_message(format!("collision at index {}.. continuing linear search", idx), 2);

		    idx += 1;
		}

		let load_factor: f32 = (self.get_item_count() as f32)/(self.m_array_size.load() as f32);

		if self.load_factor_thres < load_factor {
			HashTable::log_message(format!("Resize required. load factor={}", load_factor), 2);
			self.resize();
		}

    }


  //   pub fn manipulate_vec(&self) {

  //   	println!("enter manipulate");
  //   	self.print_ht_contents();
		// println!();

		// // let mut new_vec : &Vec<Entry> = &mut self.m_entries;
		// // let locked_entries = Arc::new(Mutex::new(new_vec));

		// // new_vec.clear();


		// let mut new_array_size = HashTable::next_power_of_2_double_the(self.m_array_size.load());

		// // my_vec has been updated to a clone of m_entries with same size
		// // m_entries elements have been updated to 0
		// // 

		// self.print_ht_contents();
		// println!();		

		// // self.m_entries.resize_with(HashTable::u32_to_usize(new_array_size), Default::default);
  //   	// let mut my_vec: Rc<Vec<Rc<Entry>>> = Rc::new(Vec::new());

  //   	// for entry in &self.m_entries {
  //   	// 	my_vec.push(entry.clone());
  //   	// 	entry.key.store(0, Ordering::Relaxed);
  //   	// 	entry.value.store(0, Ordering::Relaxed);

  //   	// 	// self.m_entries[HashTable::u32_to_usize(idx)].key.store(0, Ordering::Relaxed);
  //   	// 	// self.m_entries[HashTable::u32_to_usize(idx)].value.store(0, Ordering::Relaxed);
  //   	// }


  //   	// println!("my_vec length={}, first elem=({},{})", my_vec.len(), my_vec[0].key.load(Ordering::Relaxed), my_vec[0].value.load(Ordering::Relaxed));

  //   	self.print_ht_contents();
		// println!();

		// println!("exit manipulate");

  //   }


    // fn resize(&self, this: Arc<Mutex<HashTable>>) {
    fn resize(&self) {

    	// println!("enter resize");
    	let mut new_array_size = HashTable::next_power_of_2_double_the(self.m_array_size.load());
    	// println!("Power of 2 for {} is {}", self.m_array_size, new_array_size);

    	let mut new_vec: Vec<Entry> = Vec::new();
		for _ in 0..new_array_size {
		    new_vec.push(Entry::new());
		}

		// clear m_entries
		// resize m_entries
		// let mut new_vec = self.m_entries.to_vec();
		// self.m_entries.resize(new_array_size, Entry::new());
		// try to replace resize with some type of cloning
		{
			let mut lockedw_entries = self.m_entries.write().unwrap();
			new_vec = lockedw_entries.to_vec();
			lockedw_entries.clear();
			lockedw_entries.resize_with(HashTable::u32_to_usize(new_array_size), Default::default);
		}

		new_array_size = self.m_array_size.swap(new_array_size);
		self.item_count.swap(0);
		// self.print_ht_contents();
		// println!();

		// m_entries swapped with new_vec
		// m_array_size swapped with new_array_size

		for idx in 0..new_array_size {
			let probed_key = new_vec[HashTable::u32_to_usize(idx)].key.load(Ordering::Relaxed);
			if probed_key != 0 && probed_key != TOMBSTONE {
				let probed_value = new_vec[HashTable::u32_to_usize(idx)].value.load(Ordering::Relaxed);
				self.set_item(probed_key, probed_value);
			}

		}
		// self.print_ht_contents();
		// println!();
		// println!("exit resize");
    }



    //Retrieves an item from the hashtable given a key. Returns the value if found, 0 if not found
    pub fn get_item(&self, key:u32) -> u32 {

		assert!(key != 0 || key != TOMBSTONE);
		let mut idx = HashTable::integer_hash(key);
		// let mut idx = key.hash();
		// let mut idx = hash(&key);

		loop {
		    idx &= self.m_array_size.load() - 1;

        	// let mut loaded_entries = self.m_entries.into_inner();
        	let lockedr_entries = self.m_entries.read().unwrap();

		    let probed_key = lockedr_entries[HashTable::u32_to_usize(idx)].key.load(Ordering::Relaxed);
		    if probed_key == key {
				return lockedr_entries[HashTable::u32_to_usize(idx)].value.load(Ordering::Relaxed);
		    }
		    if probed_key == 0 || probed_key == TOMBSTONE {
				return 0
		    }

		    idx += 1;
		}
    }


    //Removes(tombstones) an item from the hashtable given a key. Returns the value if found, 0 if not found
    pub fn remove_item(&mut self, key:u32) -> u32 {
    	assert!(key != 0 || key != TOMBSTONE);
    	let mut idx = HashTable::integer_hash(key);

    	// let mut loaded_entries = self.m_entries.into_inner();

    	loop {
    		idx &= self.m_array_size.load() - 1;
    		let lockedr_entries = self.m_entries.read().unwrap();

		    let probed_key = lockedr_entries[HashTable::u32_to_usize(idx)].key.load(Ordering::Relaxed);
		    if probed_key == key {
		    	lockedr_entries[HashTable::u32_to_usize(idx)].key.store(TOMBSTONE, Ordering::Relaxed);
		    	HashTable::log_message(format!("removed key {} at index {}", key, idx), 2);
		    	self.item_count.swap(self.item_count.load()-1);
				return lockedr_entries[HashTable::u32_to_usize(idx)].value.load(Ordering::Relaxed);
		    }
		    if probed_key == 0 || probed_key == TOMBSTONE {
				return 0
		    }

		    idx += 1;	
    	}
    }


    


    fn next_power_of_2_double_the(num: u32) -> u32 {
    	let mut v = num*2;
    	v -= 1;
    	v |= v >> 1;
    	v |= v >> 2;
    	v |= v >> 4;
    	v |= v >> 8;
    	v |= v >> 16;
    	v += 1;
    	v
    }


    //a couple utility functions for size conversion
    fn u32_to_usize(key: u32) -> usize {
		key.try_into().unwrap()
    }

    fn usize_to_u32(key: usize) -> u32 {
		key.try_into().unwrap()
    }

    //----------------DEBUG-------------------
    //a dumb little function cause I can't figure out how
    //to do debug statements when compiling with cargo
    //set the if to true for debug messages
    fn log_message(msg: String, indent_lvl: u32) {
		if false {
		    for _ in 0..indent_lvl {
				print!("\t");
		    }
		    println!("{}", msg);
		}
    }

    pub fn print_ht_contents(&self) {

    	// let mut loaded_entries = self.m_entries.into_inner();
    	let lockedr_entries = self.m_entries.read().unwrap();

		for i in 0..self.m_array_size.load() {
		    print!("{}:{}, ", lockedr_entries[HashTable::u32_to_usize(i)].key.load(Ordering::Relaxed), lockedr_entries[HashTable::u32_to_usize(i)].value.load(Ordering::Relaxed))
		}
    }
    //-------------END DEBUG-------------------


    pub fn get_array_size(&self) -> u32 {
		self.m_array_size.load()
    }


  //   pub fn item_count_dec(&mut self) {
		// self.item_count -= 1;
  //   }

  //   pub fn item_count_inc(&mut self) {
		// self.item_count += 1;
  //   }


    pub fn get_item_count(&self) -> u32 {
  //   	let mut count = 0;
  //   	for idx in 0..self.get_array_size() {
  //   		if self.m_entries[HashTable::u32_to_usize(idx)].key.load(Ordering::Relaxed) != 0 && self.m_entries[HashTable::u32_to_usize(idx)].value.load(Ordering::Relaxed) != 0 {
  //   			count += 1;
  //   		}

  //   	}
		// count
		self.item_count.load()
    }


    //clear the memory and reinitialize the vector
    //TODO: get this to work with Arc
    pub fn clear(&mut self){

    	// let mut loaded_entries = self.m_entries.into_inner();

    	// Rc::make_mut(&mut self.m_entries).clear();
    	{
	    	let mut lockedw_entries = self.m_entries.write().unwrap();

			lockedw_entries.clear();
			assert!(lockedw_entries.is_empty());

			let mut my_vec: Vec<Entry> = Vec::new();
			for _ in 0..self.m_array_size.load() {
			    my_vec.push(Entry::new());
			}
			*lockedw_entries = my_vec;
		}
    }
}
