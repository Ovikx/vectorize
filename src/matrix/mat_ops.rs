use crate::Vector;

use crate::Matrix;

impl<T: Copy> Matrix<T> {
    /// Flattens a matrix into a 1D Vector
    pub fn flatten(&self) -> Vector<T> {
        let mut vector: Vector<T> = Vector::new();
        for row in self.iter() {
            for elem in row.iter() {
                vector.push(*elem)
            }
        }

        vector
    }
}