#[derive(Clone, PartialEq)]
pub struct Vector {
    values: Box<[f64]>
}


impl Vector {
    pub fn dimensions(&self) -> usize { self.values.len() }

    pub fn new(dimensions: usize, formation: impl Fn(usize)->f64) -> Vector {
        let mut values = Vec::with_capacity(dimensions);
        for i in 0..dimensions {
            values.push(formation(i))
        }
        let values = values.into_boxed_slice();
        Vector { values }
    }

    pub fn zero(dimensions: usize) -> Vector {
        Self::new(dimensions, |_| 0.0)
    }

    pub fn repeat(dimensions: usize, value: impl Into<f64> + Clone) -> Vector {
        Self::new(dimensions, |_| value.clone().into())
    }

    pub fn from_vec(values: Vec<impl Into<f64> + Clone>) -> Vector {
        Self::new(values.len(), |i| values[i].clone().into())
    }

    pub fn norm(&self) -> f64 {
        let mut norm = 0.0;
        for v in self.values.into_iter() {
            norm += v * v;
        }
        f64::sqrt(norm)
    }

    pub fn normalize(self) -> Vector {
        self.clone()*(1.0/self.norm())
    }
}


impl std::ops::Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index-1]
    }
}


use std::fmt::{ self, Debug, Formatter };
impl Debug for Vector {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.values)
    }
}


impl std::ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Self::Output {
        if self.dimensions() != other.dimensions() { panic!("Unmatching vectors dimensions") }
        Vector::new(self.dimensions(), |i| self[i+1] + other[i+1])
    }
}


impl std::ops::Mul<Vector> for Vector {
    type Output = f64;
    fn mul(self, other: Vector) -> Self::Output {
        if self.dimensions() != other.dimensions() { panic!("Unmatching vectors dimensions") }
        let mut product = 0f64;
        for i in 1..self.dimensions()+1 {
            product += self[i]*other[i]
        }
        product
    }
}


impl <T: Into<f64> + Clone> std::ops::Mul<T> for Vector {
    type Output = Vector;
    fn mul(self, scalar: T) -> Self::Output {
        Vector::new(self.dimensions(), |i| self[i+1]*scalar.clone().into())
    }
}


use crate::matrix::Matrix;
impl std::ops::Mul<Matrix> for Vector {
    type Output = Vector;
    fn mul(self, m: Matrix) -> Self::Output {
        Vector::new(m.cols(), |index| {
            let mut val = 0.0;
            for i in 0..m.lines() {
                val += self[i+1] * m[(i+1, index+1)];
            }
            val
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm() {
        let v = Vector::from_vec(vec!(1, 2, 4, 2));
        println!("{:?}", v.norm());
        let v = v.normalize();
        println!("{:?}", v);
        println!("{:?}", v.norm()==1.0);
    }

    #[test]
    fn test_v_x_m() {
        let v = Vector::from_vec(vec!(1, 0, 1));
        let m = Matrix::from_vec(3, 2, vec!(
            1, 2,
            2, 1,
            1, 2
        ));
        assert_eq!(v * m, Vector::from_vec(vec!(2, 4)));
    }
}
