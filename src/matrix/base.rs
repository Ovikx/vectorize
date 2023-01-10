use std::ops::{Deref, DerefMut};
use core::fmt::Debug;

use crate::{Vector, vector};

#[derive(Clone)]
pub struct Matrix<T> (pub Vector<Vector<T>>);

impl<T> Deref for Matrix<T> {
    type Target = Vector<Vector<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Clone> Matrix<T> {
    /// Instantiates a new Matrix from the given data
    /// 
    /// Args:
    /// - `data`: 2-dimensional Vector
    /// 
    /// Returns: Matrix
    pub fn new(data: &Vector<Vector<T>>) -> Self {
        Matrix(data.clone())
    }

    /// Returns the shape of the matrix
    /// 
    /// Returns: Tuple (`rows`: usize, `cols`: usize)
    pub fn shape(&self) -> (usize, usize) {
        let rows = self.len();
        let cols = match self.get(0) {
            Some(row) => row.len(),
            None => 0
        };

        (rows, cols)
    }

    /// Returns the matrix as `Vec<Vec<T>>`
    pub fn to_vec(&self) -> Vec<Vec<T>> {
        let mut outer_vec: Vec<Vec<T>> = vec![];
        for vector in self.iter() {
            outer_vec.push(vector.to_vec());
        }

        outer_vec
    }
}

impl<T: Clone + Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut strs: Vec<String> = vec![String::from("[")];
        for vector in &***self {
            strs.push(format!("\t{:?}", vector));
        }
        strs.push(String::from("]"));
        write!(f, "{:}", strs.join("\n"))
    }
}

impl Matrix<f64> {
    /// Creates a Matrix filled with ones as floats
    /// 
    /// Args:
    /// - `rows`: Integer, number of rows
    /// - `cols`: Integer, number of columns
    /// 
    /// Returns: Matrix of floats
    pub fn ones(rows: u32, cols: u32) -> Self {
        Matrix::fill(1.0, rows, cols)
    }

    /// Creates a Matrix filled with a given value
    /// 
    /// Args:
    /// - `value`: Float, the value to fill the matrix with
    /// - `rows`: Integer, the number of rows
    /// - `cols`: Integer, the number of columns
    /// 
    /// Returns: Matrix containing `value`'s
    pub fn fill(value: f64, rows: u32, cols: u32) -> Self {
        let mut outer_vector: Vector<Vector<f64>> = vector![];
        for _row in 0..rows {
            let mut inner_vector: Vector<f64> = vector![];
            for _col in 0..cols {
                inner_vector.push(value);
            }
            outer_vector.push(inner_vector);
        }

        Matrix(outer_vector)
    }

    /// Creates an identity matrix of a given size
    /// 
    /// Args:
    /// - `size`: Integer, size of the matrix
    /// 
    /// Returns: Identity Matrix
    pub fn identity(size: i32) -> Self {
        let mut outer_vector: Vector<Vector<f64>> = vector![];
        for row in 0..size {
            let mut inner_vector: Vector<f64> = vector![];
            for col in 0..size {
                inner_vector.push(if row==col {1.0} else {0.0});
            }
            outer_vector.push(inner_vector);
        }

        Matrix(outer_vector)
    }

    /// Returns a matrix containing random floats in the range [0.0 - 1.0)
    pub fn noise(rows: u32, cols: u32) -> Self {
        Vector::noise(rows*cols).unflatten(rows, cols)
    }
}