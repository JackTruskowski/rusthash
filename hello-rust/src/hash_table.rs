use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::convert::TryInto;


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

pub struct HashTable {
    //size must be known at compile-time for rust arrays
    //Vectors appear to be how to do Java-style arrays
    m_entries: Vec<Entry>,
    m_array_size: u32,
}

impl HashTable {

    //constructor
    pub fn new(max_size: u32) -> Self {
	assert!((max_size & (max_size -1)) == 0);

	let mut my_vec: Vec<Entry> = Vec::new();
	for _ in 0..max_size {
	    my_vec.push(Entry::new());
	}

	Self {
	    m_entries: my_vec,
	    m_array_size: max_size
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


    pub fn set_item(&self, key:u32, value:u32) {

	//0 reserved for 'empty' value
	assert!(key != 0);
	assert!(value != 0);

	let mut idx = HashTable::integer_hash(key);
	loop {

	    //scale to size of array
	    idx &= self.m_array_size - 1;

	    let result_key = self.m_entries[HashTable::u32_to_usize(idx)].key.compare_and_swap(0, key, Ordering::Relaxed);

	    if result_key == 0 || result_key == key {
		self.m_entries[HashTable::u32_to_usize(idx)].value.store(value, Ordering::Relaxed);
		HashTable::log_message(format!("added value {} at index {}", value, idx), 2);
		break;
	    }

	    HashTable::log_message(format!("collision at index {}.. continuing linear search", idx), 2);

	    idx += 1;
	}
    }

    //Retrieves an item from the hashtable given a key. Returns the value if found, 0 if not found
    pub fn get_item(&self, key:u32) -> u32 {

	assert!(key != 0);
	let mut idx = HashTable::integer_hash(key);

	loop {
	    idx &= self.m_array_size - 1;

	    let probed_key = self.m_entries[HashTable::u32_to_usize(idx)].key.load(Ordering::Relaxed);
	    if probed_key == key {
		return self.m_entries[HashTable::u32_to_usize(idx)].value.load(Ordering::Relaxed);
	    }
	    if probed_key == 0 {
		return 0
	    }

	    idx += 1;
	}
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
	    print!("{}, ", self.m_entries[HashTable::u32_to_usize(i)].value.load(Ordering::Relaxed))
	}
    }
    //-------------END DEBUG-------------------


    pub fn get_item_count(&self) -> u32 {
	self.m_array_size
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
