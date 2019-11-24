mod hash_table;

fn main() {

    let ht = hash_table::HashTable::new(4);
    println!("{}", ht.get_item_count());
    ht.set_item(1,1);


}
