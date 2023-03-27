use num_traits::Num;
use std::ops::{Add, Mul};

trait MVec<T: Num> {
    /// Dot product
    fn dot(&self, rhs: &Self) -> T;
    /// scalar mult
    fn mult(&self, scalar: T) -> Self;
    /// add
    fn add(&self, rhs: &Self) -> Self;
}

/// A vector of n-dimensions (shape of [dimensions])
pub struct Vector<T>
where
    T: Num,
{
    pub dimensions: usize,
    pub vec: Box<[T]>,
}

impl<T: Num> Vector<T> {
    pub fn new(v: Box<[T]>) -> Self {
        Vector {
            dimensions: v.len(),
            vec: v,
        }
    }
}

// pub struct Matrix<T: Num> {
//     pub shape: (usize, usize),
//     pub mat: [Vector<T>],
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector() {
        let v = [1.0, 2.0, 3.0f64];
        let v3 = Vector::new(Box::new(v));
        let x_ = [2.0, 2.0, 1.0f64];
        let x3 = Vector::new(Box::new(x_));
    }
}
