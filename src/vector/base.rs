use std::ops::{Deref, DerefMut};

use num::Float;

#[derive(Debug, Clone)]
pub struct Vector<T> (pub Vec<T>);

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Clone> Vector<T> {
    pub fn new(data: &Vec<T>) -> Self {
        Vector(data.to_vec())
    }

    pub fn read(&self) -> Vec<T> {
        (**self).clone()
    }
}

impl Vector<f64> {
    pub fn ones(size: i32) -> Self {
        let mut vec: Vec<f64> = vec![];

        for _ in 0..size {
            vec.push(1.0 as f64);
        }

        Vector::new(&vec)
    } 

    pub fn zeros(size: i32) -> Self {
        let mut vec: Vec<f64> = vec![];

        for _ in 0..size {
            vec.push(0.0 as f64);
        }

        Vector::new(&vec)
    }
}