pub mod vector;
pub mod matrix;
pub use vector::base::Vector;
pub use matrix::base::Matrix;
pub use vector::macros;

#[cfg(test)]
mod tests {
    /* use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    } */
}
