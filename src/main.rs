mod vector;
mod utils;

use crate::vector::base::Vector;

fn main() {
    let vec1 = vec![true, false, true];
    let mut vector = Vector::new(Some(&vec1));
    println!("Vector: {:?}", &vector);
    vector.push(true);
    println!("Vector: {:?}", &vector);
}