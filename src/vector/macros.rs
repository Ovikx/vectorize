#[macro_export]
macro_rules! vector {
    ($($elem:expr), *) => {
        Vector(vec![$($elem), *])
    }
}