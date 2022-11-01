#[derive(Debug, Clone)]
pub struct Vector<T> {
    pub data: Vec<T>
}

impl<T: Clone> Vector<T> {
    pub fn new(data: &Vec<T>) -> Self {
        Vector { data: data.to_vec() }
    }

    pub fn read(&self) -> Vec<T> {
        self.data.clone()
    }

    pub fn push(&mut self, elem: T) {
        self.data.push(elem);
    }
}