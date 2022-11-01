mod core;
mod utils;

use crate::core::vector::Vector;

fn main() {
    let vec1 = vec![1.1, 1.1, 1.1];
    let vector = Vector::new(Some(&vec1));
    let v2 = vector.clone() * 2.0;
    let v3 = vector.clone() + 2.0;
    println!("Vector: {:?}", &vector);
    println!("Vector * 2.0 = {:?}", &v2);
    println!("Vector + 2.0 = {:?}", &v3);
}