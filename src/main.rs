#[macro_use] extern crate itertools;

fn main() {
    //use itertools::Itertools;
    let char = "abcdefghijklmnopqrstuvwxyz";
    for (i, j, k, l, n, m, o) in iproduct!(char.chars(), char.chars(), char.chars(), char.chars(), char.chars(), char.chars(), char.chars()) {
       println!("{}{}{}{}{}{}{}", i,j,k,l,n,m,o)
    }
}
