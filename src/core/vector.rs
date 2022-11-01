use std::ops::{Mul, MulAssign, AddAssign, Add};
use num::{Integer, Float, Num};

#[derive(Debug, Clone)]
pub struct Vector<T> {
    pub content: Vec<T>
}

impl<T: Clone> Vector<T> {
    pub fn new(content: Option<&Vec<T>>) -> Self {
        match content {
            Some(c) => Vector { content: c.to_vec() },
            None => Vector { content: vec![] }
        }
    }
}

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