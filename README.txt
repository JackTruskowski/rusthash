@author Sapan Gupta
@author John Truskowski
12/18/19

--------------------------------------------------------------------------------
To run the code: (this assumes you have Rust installed)

--------------------------------------------------------------------------------

1) Edit the top of main.rs and hash_table.rs that is marked @todo.
   Here you can change the key and value sizes
   -- Default is (32bit, 32bit)
2) Edit the correct hash function in set_item() and get_item() in
   hash_table.rs (also marked with @todos)
   -- Default is 32bit

2) From inside hello-rust/ , run:

$ cargo run --bin hello-rust --release out.csv

This will write out the throughput results for insertions and finds to the
console, and out.csv.


--------------------------------------------------------------------------------
To visualize the results:

--------------------------------------------------------------------------------

From inside the charts directory, run:

$ python makecharts.py

This will produce .png files in the current directory. This requires out.csv
to exist in the hello-rust/ directory

To visualize the other results, you can run

$ python keysizes.py
or
$ python valsizes.py

These will use the data from the Google Cloud instance in the data/ directory
to generate plots for different key/value combinations.


--------------------------------------------------------------------------------
Sample outputs can be found in out.csv and the charts/figs/ directory.
out.csv was created on a 6 thread machine, so it looks weird. Other examples
of data generated from the Google Cloud instance can be found in charts/data/
