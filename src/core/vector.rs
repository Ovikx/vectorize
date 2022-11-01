use std::ops::Mul;
use num::Float;

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



impl<T: Float> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Vector<T> {
        let mut res_vec: Vector<T> = Vector { content: vec![] };
        for item in self.content {
            res_vec.content.push(item * rhs);
        }

        res_vec
    }
}