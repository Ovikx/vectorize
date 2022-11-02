use std::ops::{Mul, MulAssign, AddAssign, Add, Sub, Neg};
use num::Float;
use crate::vector::base::Vector;

impl<T: Float + MulAssign> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(mut self, rhs: T) -> Self {
        for item in &mut *self {
            *item *= rhs;
        }

        self
    }
}

impl<T: Float + MulAssign> Mul<Vector<T>> for f64 {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Vector<T> {
        rhs * match T::from(self) {
            Some(n) => n,
            None => panic!("Float could not be converted to generic")
        }
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
        for item in &mut *self {
            *item += rhs;
        }

        self
    }
}

impl <T: Float + AddAssign> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self {
        let n_iters = std::cmp::max(self.len(), rhs.len());
        let mut sum_vec: Vec<T> = vec![];

        for i in 0..n_iters {
            let first = match self.get(i) {
                Some(n) => *n,
                None => T::zero()
            };

            let second = match rhs.get(i) {
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