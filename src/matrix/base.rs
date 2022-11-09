use std::ops::{Deref, DerefMut};

use crate::{Vector, vector};

#[derive(Debug, Clone)]
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
    pub fn new(data: &Vector<Vector<T>>) -> Self {
        Matrix(data.clone())
    }
}

impl Matrix<f64> {
    pub fn ones(rows: u32, cols: u32) -> Self {
        Matrix::fill(1.0, rows, cols)
    }

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
}