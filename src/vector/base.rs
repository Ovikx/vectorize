use std::ops::{Deref, DerefMut};

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