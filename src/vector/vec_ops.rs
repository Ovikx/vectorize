use std::ops::AddAssign;

use crate::vector::base::Vector;
use num::Float;

impl<T: Float + AddAssign> Vector<T> {
    
    pub fn magnitude(&self) -> T {
        let mut total = T::zero();
        for n in &self.0 {
            total += *n;
        }

        total.sqrt()
    }

    pub fn avg(&self) -> T {
        let mut total = T::zero();
        for n in &self.0 {
            total += *n;
        }

        total / (match T::from(*&self.0.len()) {
            Some(n) => n,
            None => panic!("Could not get the length of the Vector")
        })
    }

    pub fn dot(&self, other: &Vector<T>) -> T {
        if self.0.len() != other.0.len() {
            panic!("Cannot compute dot product of two vectors of unequal length");
        }

        let u = &self.0;
        let v = &other.0;
        let mut total = T::zero();

        for i in 0..u.len() {
            total += u[i]*v[i];
        }

        total
    }

    pub fn cross(&self, other: &Vector<T>) -> Self {
        if self.0.len() != other.0.len() {
            panic!("Cannot compute cross product of two vectors of unequal length");
        }

        let u = &self.0;
        let v = &other.0;

        Vector::new(&vec![
            u[1]*v[2] - u[2]*v[1],
            -(u[0]*v[2] - u[2]*v[0]),
            u[0]*v[1] - u[1]*v[0]
        ])
    }

    pub fn abs(&self) -> Self {
        let mut abs_vec: Vec<T> = vec![];
        for num in &self.0 {
            abs_vec.push(num.abs());
        }

        Vector::new(&abs_vec)
    }
}