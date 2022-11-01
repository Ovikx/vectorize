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