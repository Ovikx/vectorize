mod vector;
mod utils;

use crate::vector::base::Vector;

fn main() {
    let v1 = Vector::new(&vec![1.1, 0.0, -3.5]);
    let v2 = v1.clone() * 2.0;
    let v3 = v1.clone() + v1.clone();
    let v4 = -Vector::new(&vec![0.1, 23.4, 234.0, -23.0]);
    println!("Vector 1: {:?}", &v1);
    println!("Vector 2: {:?}", &v2);
    println!("Vector 3: {:?}", &v3);
    println!("Vector 4: {:?}", &v4);
}