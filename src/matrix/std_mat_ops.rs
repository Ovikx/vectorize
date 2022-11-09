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