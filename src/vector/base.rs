#[derive(Debug, Clone)]
pub struct Vector<T> {
    pub content: Vec<T>
}

impl<T: Clone> Vector<T> {
    pub fn new(content: &Vec<T>) -> Self {
        Vector { content: content.to_vec() }
    }

    pub fn push(&mut self, elem: T) {
        self.content.push(elem);
    }
}