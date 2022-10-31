use std::ops::{Mul, Deref};

pub enum VectorDType<T> {
    UInt(Vector<usize>),
    Int(Vector<isize>),
    Float(Vector<f64>),
    String(Vector<String>),
    Boolean(Vector<bool>),
    Vector(Vector<T>)
}

pub enum Number {
    UInt(usize),
    Int(isize),
    Float(f64)
}


#[derive(Debug)]
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

impl Mul<f64> for Vector<Number> {
    type Output = Vector<f64>;

    fn mul(self, rhs: f64) -> Vector<f64> {
        let mut res_vec: Vector<f64> = Vector { content: vec![] };
        for item in self.content {
            res_vec.content.push(match item {
                Number::Float(n) => n * rhs,
                Number::UInt(n) => n as f64 * rhs,
                Number::Int(n) => n as f64 * rhs
            });
        }

        res_vec
    }
}