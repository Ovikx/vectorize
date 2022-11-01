use std::ops::{Mul, MulAssign, AddAssign, Add, Sub, Neg};
use num::Float;
use crate::vector::base::Vector;

impl<T: Float + MulAssign> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(mut self, rhs: T) -> Self {
        for item in &mut self.data {
            *item *= rhs;
        }

        self
    }
}

impl<T: Float + MulAssign> Neg for Vector<T> {
    type Output = Vector<T>;

    fn neg(self) -> Self {
        self * T::neg(T::one())
    }
}

impl <T: Float + AddAssign> Add<T> for Vector<T> {
    type Output = Vector<T>;

    fn add(mut self, rhs: T) -> Self {
        for item in &mut self.data {
            *item += rhs;
        }

        self
    }
}

impl <T: Float + AddAssign> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self {
        let n_iters = std::cmp::max(self.data.len(), rhs.data.len());
        let mut sum_vec: Vec<T> = vec![];

        for i in 0..n_iters {
            let first = match self.data.get(i) {
                Some(n) => *n,
                None => T::zero()
            };

            let second = match rhs.data.get(i) {
                Some(n) => *n,
                None => T::zero()
            };

            sum_vec.push(first + second);
        }

        Vector::new(&sum_vec)
    }
}

impl <T: Float + AddAssign + MulAssign> Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self {
        self + rhs * T::neg(T::one())
    }
}