use std::ops::{MulAssign, Mul};
use num::Float;
use crate::Matrix;

impl<T: Float + MulAssign + Mul> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(mut self, rhs: T) -> Self {
        for vector in &mut **self {
            *vector *= rhs;
        }

        self
    }
}

impl<T: Float + MulAssign> Mul<Matrix<T>> for f64 {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Matrix<T> {
        rhs * match T::from(self) {
            Some(n) => n,
            None => panic!("Float could not be converted to generic")
        }
    }
}