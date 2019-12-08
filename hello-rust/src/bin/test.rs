
// use hash_table;
mod hash_table_restriction1;
// use crate::hash_table;
extern crate atomic_traits;
extern crate fasthash;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::u32;


fn main() {

	// test0();
	test1();
	// test2();
}


fn test0() {

	println!("test0");
	let a  = AtomicU32::new(u32::MAX);
	let mut res = a.compare_and_swap(0, 10, Ordering::Relaxed);

	if res == u32::MAX {
		res = a.compare_and_swap(u32::MAX, 10, Ordering::Relaxed);
	}

	println!("returned:{}, saved:{}", res, a.load(Ordering::Relaxed)); 
}


fn test2() {
	let a = 12;
	println!("Power of 2 for {} is {}", a, next_power_of_2_double_the(a));
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


fn test1() {

	println!("test1");

	let mut ht: hash_table_restriction1::HashTable = hash_table_restriction1::HashTable::new(8, 0.5);
	ht.print_ht_contents();
	println!("Inserting {}", 1);
	ht.set_item(1, 9);
	println!("Get item count = {}", ht.get_item_count());
	println!("Inserting {}", 10);
	ht.set_item(10, 19);
	println!("Inserting {}", 32);
	ht.set_item(32, 90);
	println!("Inserting {}", 23);
	ht.set_item(23, 91);
	println!("Get item count = {}", ht.get_item_count());
	// println!("Hash = {}", hash_table_restriction1::hash1(33));
	println!();
	ht.print_ht_contents();
	println!();

	println!("Value for key:{} is {}", 10, ht.get_item(10));
	println!("Remove key:{}", 10);
	ht.print_ht_contents();
	println!();
	println!("Get item count = {}", ht.get_item_count());

	println!("Inserting {}", 10);
	ht.set_item(10, 190);
	ht.print_ht_contents();
	println!();
	println!("Get item count = {}", ht.get_item_count());

	println!("Inserting {}", 15);
	ht.set_item(15, 190);
	ht.print_ht_contents();
	println!();
	println!("Get item count = {}", ht.get_item_count());

	println!("Value for key:{} is {}", 15, ht.get_item(15));
	println!("Remove key:{}", 15);
	println!("Value for key:{} is {}", 15, ht.remove_item(15));
	ht.print_ht_contents();
	println!();
	println!("Value for key:{} is {}", 15, ht.get_item(15));
	println!("Get item count = {}", ht.get_item_count());


	println!("Inserting {}", 12);
	ht.set_item(12, 19);
	println!("Inserting {}", 13);
	ht.set_item(13, 90);
	println!("Inserting {}", 14);
	ht.set_item(14, 91);
	println!("Inserting {}", 16);
	ht.set_item(16, 19);
	println!("Inserting {}", 17);
	ht.set_item(17, 90);
	println!("Inserting {}", 18);
	ht.set_item(18, 91);
	println!("Get item count = {}", ht.get_item_count());




	println!("Value for key:{} is {}", 15, ht.get_item(15));
	println!("Remove key:{}", 15);
	println!("Value for key:{} is {}", 15, ht.remove_item(15));
	ht.print_ht_contents();
	println!();
	println!("Value for key:{} is {}", 15, ht.get_item(15));
	println!("Get item count = {}", ht.get_item_count());

	ht.set_item(15, 1);
	ht.print_ht_contents();
	println!();
	println!("Get item count = {}", ht.get_item_count());

	println!("Remove key:{}", 10);
	println!("Value for key:{} is {}", 10, ht.remove_item(10));
	ht.print_ht_contents();
	println!();
	println!("Get item count = {}", ht.get_item_count());

	println!("Inserting {}", 10);
	ht.set_item(10, 190);
	ht.print_ht_contents();
	println!();
	println!("Get item count = {}", ht.get_item_count());


	println!("Remove key:{}", 11);
	println!("Value for key:{} is {}", 11, ht.remove_item(11));
	println!("Get item count = {}", ht.get_item_count());

	// println!("Array size = {}, Item count = {}", ht.get_array_size(), ht.get_item_count());
	// println!("{}, {}, {}, {}", ht.get_item(1), ht.get_item(10), ht.get_item(32), ht.get_item(23));
	// println!("Array size = {}, Item count = {}", ht.get_array_size(), ht.get_item_count());
	// ht.clear();
	// println!("Array size = {}, Item count = {}", ht.get_array_size(), ht.get_item_count());
	// ht.print_ht_contents();
	println!();

}