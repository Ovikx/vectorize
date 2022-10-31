mod core;
use crate::core::vector::Vector;
fn main() {
    let vec1 = (0..14).collect();
    let vector = Vector::new(Some(&vec1));
    println!("Vector: {:?}", &vector);
    println!("Vector * 2.0 = {:?}", vector);
}