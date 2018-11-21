use std::fs::File;
use std::io::{BufWriter, Write};

#[macro_use] extern crate itertools;

fn main() {
    //use itertools::Itertools;
    let char = "abcdefghijklmnopqrstuvwxyz";
    let f = File::create("product_alphabet_3.txt").expect(
        "Unable to create file"
    );
    let mut f = BufWriter::new(f);
    for (i, j, k) in iproduct!(
        char.chars(), char.chars(), char.chars() 
    ) {
       let mut line = String::with_capacity(5);
       line.push(i);
       line.push(j);
       line.push(k);
       line.push_str("\n");
       f.write_all(line.as_bytes()).expect(
           "Unable to write data"
       );
    }
}
