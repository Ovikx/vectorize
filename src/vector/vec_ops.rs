use std::ops::AddAssign;

use crate::vector::base::Vector;
use num::Float;

impl<T: Float + AddAssign> Vector<T> {
    pub fn dot(&self) -> T {
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

        total.div(match T::from(self.content.len()) {
            Some(n) => n,
            None => panic!("Could not get the length of the Vector")
        })
    }
}