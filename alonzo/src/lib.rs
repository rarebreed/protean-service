use num_traits::Num;

trait MVec<T: Num> {
    /// Dot product
    fn dot(&self, rhs: &Self) -> T;
    /// scalar mult
    fn mult(&self, scalar: T) -> Self;
    // add
    //fn add(&self, rhs: &Self) -> Self;
}

#[derive(Debug)]
pub struct Vec64 {
    pub v: Box<[f64]>,
}

pub struct Matrixf64<'a> {
    pub m: &'a [[f64; 3]],
}

/// A vector of n-dimensions (shape of [dimensions])
pub struct Vector<T, const N: usize>
where
    T: Num + Copy,
{
    pub dimensions: usize,
    pub vec: [T; N],
}

impl<T, const N: usize> Vector<T, N>
where
    T: Num + Copy,
{
    pub fn new(v: [T; N]) -> Self {
        Vector {
            dimensions: v.len(),
            vec: v,
        }
    }
}

impl<T, const N: usize> MVec<T> for Vector<T, N>
where
    T: Num + Copy,
{
    fn dot(&self, rhs: &Self) -> T {
        //let v = *self.vec;
        let mut dot_prod = T::zero();
        for ind in 0..self.dimensions {
            dot_prod = dot_prod + self.vec[ind] * rhs.vec[ind]
        }
        dot_prod
    }

    fn mult(&self, scalar: T) -> Self {
        //let new_v: Vec<T> = self.vec.iter().map(|i| *i * scalar).collect();
        let mut new_v: [T; N] = [T::zero(); N];
        let mut ind = 0;
        for i in self.vec.iter() {
            new_v[ind] = *i * scalar;
            ind += 1;
        }
        Vector::new(new_v)
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
    fn test_vec64() {
        let v = Vec64 {
            v: Box::new([1.0, 2.0]),
        };
        println!("v is {v:#?}");
    }

    fn make_mat<'a>() -> Matrixf64<'a> {
        Matrixf64 {
            m: &[[1.0, 2.0, 3.0], [2.0, 1.0, 2.0]],
        }
    }

    #[test]
    fn test_mat() {
        let m = make_mat();
        let i = m.m[0][3];
        println!("i is {i}");
    }

    #[test]
    fn test_vector() {
        let v = [1.0, 2.0, 3.0f64];
        let v3 = Vector::new(v);
        let x_ = [2.0, 2.0, 1.0f64];
        let x3 = Vector::new(x_);

        //let n3 = v3 + x3;
    }
}
