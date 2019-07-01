#[derive(PartialEq, Clone)]
pub struct Matrix {
    lines: usize,
    cols: usize,
    values: Box<[Box<[f64]>]>
}


impl Matrix {
    pub fn lines(&self) -> usize { self.lines }
    pub fn cols(&self) -> usize { self.cols }
    pub fn is_square(&self) -> bool { self.lines == self.cols }

    // CREATES A MATRIX WITH DIMENSIONS lines, cols WITH THE GIVEN TRAINING LAW
    pub fn new(lines: usize, cols: usize, formation: impl Fn(usize, usize)->f64) -> Matrix {
        let mut _lines = Vec::with_capacity(lines);
        for l in 0..lines {
            let mut line = Vec::with_capacity(cols);
            for c in 0..cols {
                line.push(formation(l, c));
            }
            _lines.push(line.into_boxed_slice());
        }
        let values = _lines.into_boxed_slice();
        Matrix { lines, cols, values }
    }

    // CREATES A NULL MATRIX WITH DIMENSIONS lines, cols
    pub fn zero(lines: usize, cols: usize) -> Matrix {
        Self::new(lines, cols, |_, _| 0.0)
    }

    // CREATES AN IDENTITY MATRIX WITH SIZE size
    pub fn identity(size: usize) -> Matrix {
        Self::new(size, size, |l, c| if l == c { 1.0 } else { 0.0 })
    }

    // CREATES A MATRIX WITH DIMENSIONS lines, cols WITH THE GIVEN f64 VALUES
    pub fn from_vec(lines: usize, cols: usize, values: Vec<impl Into<f64> + Clone>) -> Matrix {
        if lines*cols != values.len() { panic!("Unmatching dimensions of given inputs") }
        Self::new(lines, cols, |l, c| { values[l*cols+c].clone().into() })
    }

    // RETURNS THE TRANSPOSE MATRIX OF self
    pub fn transpose(&self) -> Matrix {
        Self::new(self.cols(), self.lines(), |l, c| self[(c+1, l+1)].clone())
    }

    pub fn inverse(mut self) -> Matrix {
        if !self.is_square() { panic!("Not invertible matrix") }
        let mut output = Matrix::identity(self.lines());

        for _l in 1..self.lines+1 {
            let pivot = self[(_l, _l)];
            for c in 1..self.cols+1 {
                self[(_l, c)] /= pivot;
                output[(_l, c)] /= pivot;
            }
            for l in 1..self.lines+1 {
                let pivot = -1.0*self[(l, _l)];
                if l != _l {
                    for c in 1..self.cols+1 {
                        self[(l, c)] += self[(_l, c)]*pivot;
                        output[(l, c)] += output[(_l, c)]*pivot;
                    }
                }
            }
        }

        output
    }
}


// INDEXING IMPLEMENTATION WITH INDEX = (line>0, column>0)
impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.values[index.0 - 1][index.1 - 1]
    }
}
impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.values[index.0 - 1][index.1 - 1]
    }
}


use std::fmt::{ self, Debug, Formatter };
impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut output = String::new();
        for l in 0..self.lines() {
            for c in 0..self.cols() {
                output.push_str(&format!("{}     ", self[(l+1, c+1)]));
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}


// SUM BETWEEN MATRICES
impl std::ops::Add<Matrix> for Matrix {
    type Output = Result<Matrix, &'static str>;

    fn add(self, other: Matrix) -> Self::Output {
        if (self.lines(), self.cols()) != (other.lines(), other.cols()) {
            return Err("Unmatching matrices dimensions")
        }
        Ok(Matrix::new(
            self.lines(),
            self.cols(),
            |l, c| { self[(l+1, c+1)].clone() + other[(l+1, c+1)].clone() }
        ))
    }
}


// PRODUCT BETWEEN MATRICES
impl std::ops::Mul<Matrix> for Matrix {
    type Output = Result<Matrix, &'static str>;

    fn mul(self, other: Matrix) -> Self::Output {
        if self.cols() != other.lines() { return Err("Unmatching matrices dimensions") }
        Ok(Matrix::new(
            self.lines(),
            other.cols(),
            |l, c| {
                let mut value = 0.0;
                for m in 0..self.cols() {
                    value += self[(l+1, m+1)].clone() * other[(m+1, c+1)].clone();
                }
                value
            }
        ))
    }
}


use crate::vector::Vector;
impl std::ops::Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, v: Vector) -> Self::Output {
        Vector::new(self.lines(), |index| {
            let mut val = 0.0;
            for i in 0..self.cols() {
                val += self[(index+1, i+1)] * v[i+1];
            }
            val
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let m1 = Matrix::identity(3);
        let m2 = Matrix::from_vec(3, 3, vec!(
            1., 1., 1.,
            0., 0., 0.,
            1., 2., 3.,
        ));
        assert_eq!(m1 + m2, Ok(Matrix::from_vec(3, 3, vec!(
            2., 1., 1.,
            0., 1., 0.,
            1., 2., 4.
        ))));
        assert_eq!(
            Matrix::identity(2) + Matrix::zero(2, 3),
            Err("Unmatching matrices dimensions")
        );
    }

    #[test]
    fn test_product() {
        let m = Matrix::from_vec(2, 3, vec!(
            1, 2, 1,
            2, 1, 3
        ));
        let n = Matrix::from_vec(3, 1, vec!(
            1,
            5,
            1
        ));
        assert_eq!(m * n, Ok(Matrix::from_vec(2, 1, vec!(
            12,
            10
        ))));

        let m = Matrix::from_vec(3, 4, vec!(
            1, 2, 4, 2,
            1, 6, 7, 0,
            2, 8, 3, 9
        ));
        assert_eq!(m.clone()*Matrix::identity(4), Ok(m.clone()));
        assert_eq!(m*Matrix::identity(3), Err("Unmatching matrices dimensions"));
    }

    #[test]
    fn test_transpose() {
        let m = Matrix::from_vec(2, 3, vec!(
            1, 2, 3,
            4, 5, 6
        )).transpose();
        let n = Matrix::from_vec(3, 2, vec!(
            1, 4,
            2, 5,
            3, 6,
        ));
        assert_eq!(m, n);
        assert_eq!(Matrix::identity(3).transpose(), Matrix::identity(3));
    }

    #[test]
    fn test_m_x_v() {
        let m = Matrix::from_vec(2, 3, vec!(
            1, 2, 1,
            2, 1, 2
        ));
        let v = Vector::from_vec(vec!(1, 0, 1));
        assert_eq!(m * v, Vector::from_vec(vec!(2, 4)));
    }

    #[test]
    fn test_inverse() {
        let m = Matrix::from_vec(2, 2, vec!(2, 4, 6, 8));
        let i = Matrix::from_vec(2, 2, vec!(
            -1.0   , 0.5,
            3.0/4.0, -1.0/4.0
        ));
        assert_eq!(m.clone().inverse(), i);
        assert_eq!(m*i, Ok(Matrix::identity(2)));
        /*
        let m = Matrix::from_vec(3, 3, vec!(
            3, 3, 1,
            2, 1, 4,
            1, 5, 2
        ));
        let i = Matrix::from_vec(3, 3, vec!(
            18./45.,  1./45., -11./45.,
             0./45., -5./45.,  10./45.,
            -9./45., 12./45.,   3./45.
        ));
        assert_eq!(m.clone().inverse(), i);
        assert_eq!(m*i, Ok(Matrix::identity(3)));
        */
    }
}
