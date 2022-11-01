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
}