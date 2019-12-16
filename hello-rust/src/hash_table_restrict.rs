// This Hash table add remove and resize functionality to the jeff preshing code



use std::sync::atomic::AtomicU32;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::convert::TryInto;
// use std::hash::{Hash, Hasher};
// use std::collections::hash_map::DefaultHasher;
// use atomic_traits::{Atomic};
use std::u32;
use std::mem;
use std::cell::Cell;
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



//Rust port of Jeff Preshing's simple lock-free hash table
pub struct Entry {
    key: AtomicU32,
    value: AtomicU32,
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
    m_entries: Vec<Entry>,
    m_array_size: u32,
    load_factor_thres: f32,
    item_count: u32,
}

impl HashTable {

    //constructor
    pub fn new(max_size: u32, load_factor: f32) -> Self {
	assert!((max_size & (max_size -1)) == 0);

	let mut my_vec: Vec<Entry> = Vec::new();
	for _ in 0..max_size {
	    my_vec.push(Entry::new());
	}
	// self.item_count.set(0)

	Self {
	    m_entries: my_vec,
	    m_array_size: max_size,
	    load_factor_thres: load_factor,
	    item_count: 0
	}
    }



    //from https://stackoverflow.com/questions/664014/what-
    //integer-hash-function-are-good-that-accepts-an-integer-hash-key
    fn integer_hash2(mut x: u32) -> u32 {
	x = ((x >> 16) ^ x);
	x = x.wrapping_mul(0x45d9f3b);
	x = ((x >> 16) ^ x);
	x = x.wrapping_mul(0x45d9f3b);
	x = (x >> 16) ^ x;
	x
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


    pub fn set_item(&mut self, key:u32, value:u32) {

	//0 reserved for 'empty' value
	assert!(key != 0 || key != u32::MAX);
	assert!(value != 0);

	let mut idx = HashTable::integer_hash2(key);
	// let mut idx = murmur3::hash(key);
	// let mut idx = hash(&key);
	loop {

	    //scale to size of array
	    idx &= self.m_array_size - 1;


	    let mut result_key = self.m_entries[HashTable::u32_to_usize(idx)].key.compare_and_swap(0, key, Ordering::Relaxed);

	    if result_key == u32::MAX {
		result_key = self.m_entries[HashTable::u32_to_usize(idx)].key.compare_and_swap(u32::MAX, key, Ordering::Relaxed);
	    }

	    if result_key == 0 || result_key == key || result_key == u32::MAX {
		self.m_entries[HashTable::u32_to_usize(idx)].value.store(value, Ordering::Relaxed);
		HashTable::log_message(format!("added value {} at index {}", value, idx), 2);
		// self.item_count.set(self.item_count.get()+1);
		break;
	    }

	    HashTable::log_message(format!("collision at index {}.. continuing linear search", idx), 2);

	    idx += 1;
	}

	let load_factor: f32 = (self.get_item_count() as f32)/(self.m_array_size as f32);

	if self.load_factor_thres < load_factor {
	    HashTable::log_message(format!("Resize required. load factor={}", load_factor), 2);
	    self.resize();
	}

    }

    //Retrieves an item from the hashtable given a key. Returns the value if found, 0 if not found
    pub fn get_item(&self, key:u32) -> u32 {

	assert!(key != 0 || key != u32::MAX);
	let mut idx = HashTable::integer_hash2(key);
	// let mut idx = key.hash();
	// let mut idx = hash(&key);

	loop {
	    idx &= self.m_array_size - 1;

	    let probed_key = self.m_entries[HashTable::u32_to_usize(idx)].key.load(Ordering::Relaxed);
	    if probed_key == key {
		return self.m_entries[HashTable::u32_to_usize(idx)].value.load(Ordering::Relaxed);
	    }
	    if probed_key == 0 || probed_key == u32::MAX {
		return 0
	    }

	    idx += 1;
	}
    }


    //Removes(tombstones) an item from the hashtable given a key. Returns the value if found, 0 if not found
    pub fn remove_item(&mut self, key:u32) -> u32 {
    	assert!(key != 0 && key != u32::MAX);
    	let mut idx = HashTable::integer_hash(key);

    	loop {
    	    idx &= self.m_array_size - 1;

	    let probed_key = self.m_entries[HashTable::u32_to_usize(idx)].key.load(Ordering::Relaxed);
	    if probed_key == key {
		self.m_entries[HashTable::u32_to_usize(idx)].key.store(u32::MAX, Ordering::Relaxed);
		HashTable::log_message(format!("removed key {} at index {}", key, idx), 2);
		// self.item_count.set(self.item_count.get()-1);
		return self.m_entries[HashTable::u32_to_usize(idx)].value.load(Ordering::Relaxed);
	    }
	    if probed_key == 0 || probed_key == u32::MAX {
		return 0
	    }

	    idx += 1;
    	}
    }


    fn resize(&mut self) {

    	let mut new_array_size = HashTable::next_power_of_2_double_the(self.m_array_size);
    	// println!("Power of 2 for {} is {}", self.m_array_size, new_array_size);

    	let mut new_vec: Vec<Entry> = Vec::new();
	for _ in 0..new_array_size {
	    new_vec.push(Entry::new());
	}

	// self.print_ht_contents();
	// println!();


	// let mut temp_vec = self.m_entries;
	// self.m_entries.resize(new_array_size, Entry::new());



	mem::swap(&mut self.m_entries, &mut new_vec);
	// let mut temp_size = self.m_array_size;

	mem::swap(&mut self.m_array_size, &mut new_array_size);
	// self.item_count.set(0);

	// self.print_ht_contents();
	// println!();
	// println!("{:?}", new_vec.len());

	// m_entries swapped with new_vec
	// m_array_size swapped with new_array_size


	for idx in 0..new_array_size {
	    let probed_key = new_vec[HashTable::u32_to_usize(idx)].key.load(Ordering::Relaxed);
	    if probed_key != 0 && probed_key != u32::MAX {
		let probed_value = new_vec[HashTable::u32_to_usize(idx)].value.load(Ordering::Relaxed);
		self.set_item(probed_key, probed_value);
	    }

	}

	// self.print_ht_contents();
	// println!();


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
	for i in 0..self.m_array_size {
	    print!("{}:{}, ", self.m_entries[HashTable::u32_to_usize(i)].key.load(Ordering::Relaxed), self.m_entries[HashTable::u32_to_usize(i)].value.load(Ordering::Relaxed))
	}
    }
    //-------------END DEBUG-------------------


    pub fn get_array_size(&self) -> u32 {
	self.m_array_size
    }


    //   pub fn item_count_dec(&mut self) {
    // self.item_count -= 1;
    //   }

    //   pub fn item_count_inc(&mut self) {
    // self.item_count += 1;
    //   }


    pub fn get_item_count(&self) -> u32 {
    	let mut count = 0;
    	for idx in 0..self.get_array_size() {
    	    if self.m_entries[HashTable::u32_to_usize(idx)].key.load(Ordering::Relaxed) != 0 &&
		self.m_entries[HashTable::u32_to_usize(idx)].value.load(Ordering::Relaxed) != 0 {
    		count += 1;
    	    }

    	}
	count
	// self.item_count.get()
    }


    //clear the memory and reinitialize the vector
    //TODO: get this to work with Arc
    pub fn clear(&mut self){

	self.m_entries.clear();
	assert!(self.m_entries.is_empty());

	let mut my_vec: Vec<Entry> = Vec::new();
	for _ in 0..self.m_array_size {
	    my_vec.push(Entry::new());
	}
	self.m_entries = my_vec;
    }
}
