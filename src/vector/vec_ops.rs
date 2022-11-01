use std::ops::AddAssign;

use crate::vector::base::Vector;
use num::Float;

impl<T: Float + AddAssign> Vector<T> {
    pub fn magnitude(&self) -> T {
        let mut total = T::zero();
        for n in &self.content {
            total += *n;
        }

        total.sqrt()
    }

    pub fn avg(&self) -> T {
        let mut total = T::zero();
        for n in &self.content {
            total += *n;
        }

        total / (match T::from(*&self.content.len()) {
            Some(n) => n,
            None => panic!("Could not get the length of the Vector")
        })
    }

    pub fn cross(&self, other: &Vector<T>) -> Self {
        if *&self.content.len() != other.content.len() {
            panic!("Cannot compute cross product of two vectors of unequal length");
        }

        let u = &self.content;
        let v = &other.content;

        Vector::new(&vec![
            u[1]*v[2] - u[2]*v[1],
            -(u[0]*v[2] - u[2]*v[0]),
            u[0]*v[1] - u[1]*v[0]
        ])
    }
}