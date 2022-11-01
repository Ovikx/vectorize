mod vector;
mod utils;

use crate::vector::base::Vector;

fn main() {
    let mut vector = Vector::new(&vec![1.4]);
    println!("Vector: {:?}", &vector);
    vector.push(2.5);
    println!("Vector: {:?}", &vector);
    println!("Vector dot: {:?}", &vector.dot(&Vector::new(&vec![1.1, 1.1])));
    println!("Vector contents with read(): {:?}", &vector.read());
}