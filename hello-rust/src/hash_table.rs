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
    //but this isn't working yet
    m_entries: Vec<Entry>,
    m_array_size: u32,
}

impl HashTable {

    //constructor
    pub fn new(max_size: u32) -> Self {
	assert!((max_size & (max_size -1)) == 0);
	Self {
	    m_entries: vec![Entry::new(); HashTable::u32_to_usize(max_size)],
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
	//TODO
	//0 reserved for 'empty' value
	assert!(key != 0);
	assert!(value != 0);

	let mut idx = HashTable::integer_hash(key);
	loop {

	    //scale to size of array
	    idx &= self.m_array_size - 1;

	    let result_key = self.m_entries[HashTable::u32_to_usize(idx)].key.compare_and_swap(0, key, Ordering::Relaxed);

	    println!("{}", result_key);
	    break;
	    //idx+=1;

	}

    }

    pub fn get_item(&self, key:u32) -> u32 {
	//TODO
	-1
    }

    fn u32_to_usize(key: u32) -> usize {
	key.try_into().unwrap()
    }

    fn usize_to_u32(key: usize) -> u32 {
	key.try_into().unwrap()
    }

    pub fn get_item_count(&self) -> u32 {
	self.m_array_size
    }

    pub fn clear(&self){
	//TODO
    }
}
