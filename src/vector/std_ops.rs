use std::ops::{Mul, MulAssign, AddAssign, Add};
use num::Float;
use crate::vector::base::Vector;

impl<T: Float + MulAssign> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(mut self, rhs: T) -> Self {
        for item in &mut self.content {
            *item *= rhs;
        }

        self
    }
}

impl <T: Float + AddAssign> Add<T> for Vector<T> {
    type Output = Vector<T>;

    fn add(mut self, rhs: T) -> Self {
        for item in &mut self.content {
            *item += rhs;
        }

        self
    }
}