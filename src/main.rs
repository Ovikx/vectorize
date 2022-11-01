mod vector;
mod utils;

use crate::vector::base::Vector;

fn main() {
    let v1 = Vector::new(&vec![1.1, 0.0, -3.5]);
    let v2 = v1.clone() * 2.0;
    let v3 = v1.clone() + v1.clone();
    println!("Vector 1: {:?}", &v1);
    println!("Vector 2: {:?}", &v2);
    println!("Vector 3: {:?}", &v3);
}