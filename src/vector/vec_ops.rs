use std::ops::AddAssign;

use crate::vector::base::Vector;
use num::Float;

impl<T: Float + AddAssign> Vector<T> {
    
    pub fn magnitude(&self) -> T {
        let mut total = T::zero();
        for n in &self.data {
            total += *n;
        }

        total.sqrt()
    }

    pub fn avg(&self) -> T {
        let mut total = T::zero();
        for n in &self.data {
            total += *n;
        }

        total / (match T::from(*&self.data.len()) {
            Some(n) => n,
            None => panic!("Could not get the length of the Vector")
        })
    }

    pub fn dot(&self, other: &Vector<T>) -> T {
        if self.data.len() != other.data.len() {
            panic!("Cannot compute dot product of two vectors of unequal length");
        }

        let u = &self.data;
        let v = &other.data;
        let mut total = T::zero();

        for i in 0..u.len() {
            total += u[i]*v[i];
        }

        total
    }

    pub fn cross(&self, other: &Vector<T>) -> Self {
        if self.data.len() != other.data.len() {
            panic!("Cannot compute cross product of two vectors of unequal length");
        }

        let u = &self.data;
        let v = &other.data;

        Vector::new(&vec![
            u[1]*v[2] - u[2]*v[1],
            -(u[0]*v[2] - u[2]*v[0]),
            u[0]*v[1] - u[1]*v[0]
        ])
    }

    pub fn add_vec(&self, other: &Vector<T>) -> Self {
        assert_eq!(self.data.len(), other.data.len());
        let mut sum_vec: Vec<T> = vec![];

        for i in 0..self.data.len() {
            sum_vec.push(self.data[i] + other.data[i]);
        }

        Vector::new(&sum_vec)
    }
}