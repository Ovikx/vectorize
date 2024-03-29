use std::{ops::{Deref, DerefMut}, fmt::Debug};
use rand;

#[derive(Clone)]
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
    /// Instantiates a new Vector
    /// 
    /// Args:
    /// - `data`: Vec<T>
    /// 
    /// Returns: Vector<T>
    pub fn from(data: &Vec<T>) -> Self {
        Vector(data.to_vec())
    }

    /// Instantiates an empty Vector
    pub fn new() -> Self {
        Vector(vec![])
    }
}

impl<T: Clone + Debug> Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", **self)
    }
}

impl Vector<f64> {
    /// Creates a Vector of ones as floats
    /// 
    /// Args:
    /// - `size`: Integer, size of the Vector
    pub fn ones(size: i32) -> Self {
        let mut vec: Vec<f64> = vec![];

        for _ in 0..size {
            vec.push(1.0 as f64);
        }

        Vector(vec)
    } 

    /// Creates a Vector of zeros as floats
    /// 
    /// Args:
    /// - `size`: Integer, size of the Vector
    pub fn zeros(size: i32) -> Self {
        let mut vec: Vec<f64> = vec![];

        for _ in 0..size {
            vec.push(0.0 as f64);
        }

        Vector(vec)
    }

    /// Returns a Vector containing random floats in [0.0, 1.0)
    /// 
    /// ## Arguments
    /// 
    /// * `size` - The length of the vector
    pub fn noise(size: u32) -> Self {
        let mut vec = vec![];

        for _ in 0..size {
            vec.push(rand::random::<f64>());
        };

        Vector(vec)
    }
}