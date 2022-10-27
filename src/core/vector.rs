pub enum VectorDType<T> {
    UInt(Vector<usize>),
    Int(Vector<isize>),
    Float(Vector<f64>),
    String(Vector<String>),
    Boolean(Vector<bool>),
    Vector(Vector<T>)
}

#[derive(Debug)]
pub struct Vector<T> {
    pub content: Vec<T>
}

impl<T: Clone> Vector<T> {
    pub fn new(content: Option<&Vec<T>>) -> Self {
        match content {
            Some(c) => Vector { content: c.clone() },
            None => Vector { content: vec![] }
        }
    }
}