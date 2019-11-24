mod hash_table;

fn main() {

    let mut ht = hash_table::HashTable::new(4);
    ht.set_item(1,1);
    ht.set_item(1,2);
    ht.set_item(2,4);
    ht.set_item(18,21);

    println!("{}", ht.get_item(1));

    ht.clear();

    println!("{}", ht.get_item(1));



}
