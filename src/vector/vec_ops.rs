use std::ops::AddAssign;

use crate::vector::base::Vector;
use num::Float;

impl<T: Float + AddAssign> Vector<T> {
    /// Computes the magnitude of the vector
    pub fn magnitude(&self) -> T {
        let mut total = T::zero();
        for n in &**self {
            total += *n;
        }

        total.sqrt()
    }

    /// Computes the average value of the vector
    pub fn avg(&self) -> T {
        let mut total = T::zero();
        for n in &**self {
            total += *n;
        }

        total / (match T::from(self.len()) {
            Some(n) => n,
            None => panic!("Could not get the length of the Vector")
        })
    }

    /// Computes the dot product of this vector and another vector
    /// 
    /// Args:
    /// - `other`: Vector of floats
    /// 
    /// Returns: Float
    /// 
    /// # Example
    /// ```rust
    /// vector![a, b, c].dot(vector![c, d, e]) -> a*c + b*d + c*e
    /// ```
    pub fn dot(&self, other: &Vector<T>) -> T {
        assert_eq!(self.len(), other.len());

        let u = &**self;
        let v = &**other;
        let mut total = T::zero();

        for i in 0..u.len() {
            total += u[i]*v[i];
        }

        total
    }

    /// Computes the cross product of this vector and another vector
    /// 
    /// Args:
    /// - `other`: Vector of floats
    /// 
    /// Returns: Vector of floats
    pub fn cross(&self, other: &Vector<T>) -> Self {
        assert_eq!(self.len(), 3);
        assert_eq!(self.len(), other.len());

        let u = &**self;
        let v = &**other;

        Vector::new(&vec![
            u[1]*v[2] - u[2]*v[1],
            -(u[0]*v[2] - u[2]*v[0]),
            u[0]*v[1] - u[1]*v[0]
        ])
    }

    /// Applies absolute value onto every element in the vector
    pub fn abs(&self) -> Self {
        let mut abs_vec: Vec<T> = vec![];
        for num in &**self {
            abs_vec.push(num.abs());
        }

        Vector::new(&abs_vec)
    }
}