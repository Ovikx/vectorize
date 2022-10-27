mod core;
use crate::core::vector::Vector;
fn main() {
    let vec1 = vec![1,2,3,4];
    let vector = Vector::new(Some(&vec1));
    println!("Vector: {:?}", &vector);
}