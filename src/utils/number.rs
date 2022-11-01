pub enum Number {
    UInt(usize),
    Int(isize),
    Float(f64)
}

impl From<usize> for Number {
    fn from(num: usize) -> Self {
        Number::Float(num as f64)
    }
}

impl From<isize> for Number {
    fn from(num: isize) -> Self {
        Number::Float(num as f64)
    }
}

impl From<f32> for Number {
    fn from(num: f32) -> Self {
        Number::Float(num as f64)
    }
}

impl From<f64> for Number {
    fn from(num: f64) -> Self {
        Number::Float(num)
    }
}