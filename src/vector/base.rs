#[derive(Debug, Clone)]
pub struct Vector<T> (pub Vec<T>);

impl<T: Clone> Vector<T> {
    pub fn new(data: &Vec<T>) -> Self {
        Vector(data.to_vec())
    }

    pub fn read(&self) -> Vec<T> {
        self.0.clone()
    }

    pub fn push(&mut self, elem: T) {
        self.0.push(elem);
    }
}