use crate::tuple;
use std::ops;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Matrix {
    shape: (usize, usize),
    m: Vec<Vec<f64>>,
}
impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            shape: (width, height),
            m: vec![vec![0.0f64; width]; height],
        }
    }
    pub fn from(width: usize, height: usize, data: Vec<Vec<f64>>) -> Self {
        Matrix {
            shape: (width, height),
            m: data,
        }
    }
    pub fn identity() -> Self {
        Matrix::from(
            4,
            4,
            vec![
                vec![1.0, 0., 0., 0.],
                vec![0.0, 1., 0., 0.],
                vec![0.0, 0., 1., 0.],
                vec![0.0, 0., 0., 1.],
            ],
        )
    }
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Matrix::from(
            4,
            4,
            vec![
                vec![1.0, 0., 0., x],
                vec![0.0, 1., 0., y],
                vec![0.0, 0., 1., z],
                vec![0.0, 0., 0., 1.],
            ],
        )
    }
    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        Matrix::from(
            4,
            4,
            vec![
                vec![x, 0., 0., 0.],
                vec![0.0, y, 0., 0.],
                vec![0.0, 0., z, 0.],
                vec![0.0, 0., 0., 1.],
            ],
        )
    }
    pub fn rot_x(a: f64) -> Self {
        Matrix::from(
            4,
            4,
            vec![
                vec![1., 0., 0., 0.],
                vec![0.0, f64::cos(a), -f64::sin(a), 0.],
                vec![0.0, f64::sin(a), f64::cos(a), 0.],
                vec![0.0, 0., 0., 1.],
            ],
        )
    }
    pub fn rot_y(a: f64) -> Self {
        Matrix::from(
            4,
            4,
            vec![
                vec![f64::cos(a), 0., f64::sin(a), 0.],
                vec![0.0, 1., 0., 0.],
                vec![-f64::sin(a), 0., f64::cos(a), 0.],
                vec![0.0, 0., 0., 1.],
            ],
        )
    }
    pub fn rot_z(a: f64) -> Self {
        Matrix::from(
            4,
            4,
            vec![
                vec![f64::cos(a), -f64::sin(a), 0., 0.],
                vec![f64::sin(a), f64::cos(a), 0., 0.],
                vec![0., 0., 1., 0.],
                vec![0., 0., 0., 1.],
            ],
        )
    }
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Matrix::from(
            4,
            4,
            vec![
                vec![1., xy, xz, 0.],
                vec![yx, 1., yz, 0.],
                vec![zx, zy, 1., 0.],
                vec![0., 0., 0., 1.],
            ],
        )
    }
    pub fn transpose(&self) -> Self {
        let mut m = Matrix::new(4, 4);
        for y in 0..4 as usize {
            for x in 0..4 as usize {
                m[(x, y)] = self[(y, x)];
            }
        }
        m
    }

    pub fn determinant(&self) -> f64 {
        if self.shape == (2, 2) {
            self[(0, 0)] * self[(1, 1)] - self[(1, 0)] * self[(0, 1)]
        } else {
            (0..self.shape.0)
                .map(|x| self[(0, x)] * self.cofactor(0, x))
                .sum()
        }
    }

    pub fn submatrix(&self, r: usize, c: usize) -> Self {
        Matrix::from(
            self.shape.0 - 1,
            self.shape.1 - 1,
            self.m
                .iter()
                .enumerate()
                .filter_map(|(pos, e)| {
                    if pos == r {
                        None
                    } else {
                        Some(
                            e.iter()
                                .enumerate()
                                .filter_map(
                                    |(cpos, cell)| if cpos == c { None } else { Some(*cell) },
                                )
                                .collect(),
                        )
                    }
                })
                .collect(),
        )
    }
    pub fn minor(&self, r: usize, c: usize) -> f64 {
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f64 {
        if (r + c) % 2 == 1 {
            -self.minor(r, c)
        } else {
            self.minor(r, c)
        }
    }
    pub fn is_inversible(&self) -> bool {
        self.determinant() != 0f64
    }

    pub fn inverse(&self) -> Option<Self> {
        if !self.is_inversible() {
            None
        } else {
            let mut m = Matrix::new(self.shape.0, self.shape.1);
            let det = self.determinant();
            for row in 0..self.shape.0 {
                for col in 0..self.shape.1 {
                    let c = self.cofactor(row, col);
                    m[(col, row)] = c / det;
                }
            }
            Some(m)
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index<'a>(&'a self, (y, x): (usize, usize)) -> &'a f64 {
        &self.m[y][x]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut<'a>(&'a mut self, (y, x): (usize, usize)) -> &'a mut f64 {
        &mut self.m[y][x]
    }
}
impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: Matrix) -> Matrix {
        let mut m = Matrix::new(self.shape.0, self.shape.1);
        for r in 0..self.shape.0 as usize {
            for c in 0.._rhs.shape.1 as usize {
                m[(r, c)] = self[(r, 0)] * _rhs[(0, c)]
                    + self[(r, 1)] * _rhs[(1, c)]
                    + self[(r, 2)] * _rhs[(2, c)]
                    + self[(r, 3)] * _rhs[(3, c)];
            }
        }
        m
    }
}
impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: &Matrix) -> Matrix {
        let mut m = Matrix::new(self.shape.0, self.shape.1);
        for r in 0..self.shape.0 as usize {
            for c in 0.._rhs.shape.1 as usize {
                m[(r, c)] = self[(r, 0)] * _rhs[(0, c)]
                    + self[(r, 1)] * _rhs[(1, c)]
                    + self[(r, 2)] * _rhs[(2, c)]
                    + self[(r, 3)] * _rhs[(3, c)];
            }
        }
        m
    }
}
impl ops::Mul<tuple::Tuple> for Matrix {
    type Output = tuple::Tuple;
    fn mul(self, _rhs: tuple::Tuple) -> tuple::Tuple {
        tuple::Tuple {
            x: self[(0, 0)] * _rhs.x
                + self[(0, 1)] * _rhs.y
                + self[(0, 2)] * _rhs.z
                + self[(0, 3)] * _rhs.w,
            y: self[(1, 0)] * _rhs.x
                + self[(1, 1)] * _rhs.y
                + self[(1, 2)] * _rhs.z
                + self[(1, 3)] * _rhs.w,
            z: self[(2, 0)] * _rhs.x
                + self[(2, 1)] * _rhs.y
                + self[(2, 2)] * _rhs.z
                + self[(2, 3)] * _rhs.w,
            w: self[(3, 0)] * _rhs.x
                + self[(3, 1)] * _rhs.y
                + self[(3, 2)] * _rhs.z
                + self[(3, 3)] * _rhs.w,
        }
    }
}
impl ops::Mul<tuple::Tuple> for &Matrix {
    type Output = tuple::Tuple;
    fn mul(self, _rhs: tuple::Tuple) -> tuple::Tuple {
        tuple::Tuple {
            x: self[(0, 0)] * _rhs.x
                + self[(0, 1)] * _rhs.y
                + self[(0, 2)] * _rhs.z
                + self[(0, 3)] * _rhs.w,
            y: self[(1, 0)] * _rhs.x
                + self[(1, 1)] * _rhs.y
                + self[(1, 2)] * _rhs.z
                + self[(1, 3)] * _rhs.w,
            z: self[(2, 0)] * _rhs.x
                + self[(2, 1)] * _rhs.y
                + self[(2, 2)] * _rhs.z
                + self[(2, 3)] * _rhs.w,
            w: self[(3, 0)] * _rhs.x
                + self[(3, 1)] * _rhs.y
                + self[(3, 2)] * _rhs.z
                + self[(3, 3)] * _rhs.w,
        }
    }
}
#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::matrix;
    use crate::tuple;
    #[test]
    fn test_matrix_4x4() {
        let m = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![1., 2., 3., 4.],
                vec![5.5, 6.5, 7.5, 8.5],
                vec![9., 10., 11., 12.],
                vec![13.5, 14.5, 15.5, 16.5],
            ],
        );
        assert_eq!(m[(0, 0)], 1.);
        assert_eq!(m[(0, 3)], 4.);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }
    #[test]
    fn test_matrix_2x2() {
        let m = matrix::Matrix::from(2, 2, vec![vec![-3., 5.], vec![1., -2.]]);

        assert_eq!(m[(0, 0)], -3.);
        assert_eq!(m[(0, 1)], 5.);
        assert_eq!(m[(1, 0)], 1.);
        assert_eq!(m[(1, 1)], -2.);
    }
    #[test]
    fn test_matrix_3x3() {
        let m = matrix::Matrix::from(
            3,
            3,
            vec![vec![-3., 5., 0.], vec![1., -2., -7.], vec![0., 1., 1.]],
        );

        assert_eq!(m[(0, 0)], -3.);
        assert_eq!(m[(1, 1)], -2.);
        assert_eq!(m[(2, 2)], 1.);
    }
    #[test]
    fn test_matrix_eq() {
        let m1 = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        let m2 = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        assert_eq!(m1, m2);
    }
    #[test]
    fn test_matrix_neq() {
        let m1 = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        let m2 = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![2., 3., 4., 5.],
                vec![6., 7., 8., 9.],
                vec![8., 7., 6., 5.],
                vec![4., 3., 2., 1.],
            ],
        );
        assert_ne!(m1, m2);
    }

    #[test]
    fn test_matrix_mul() {
        let m1 = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![1., 2., 3., 4.],
                vec![5., 6., 7., 8.],
                vec![9., 8., 7., 6.],
                vec![5., 4., 3., 2.],
            ],
        );
        let m2 = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![-2., 1., 2., 3.],
                vec![3., 2., 1., -1.],
                vec![4., 3., 6., 5.],
                vec![1., 2., 7., 8.],
            ],
        );
        let expected = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![20., 22., 50., 48.],
                vec![44., 54., 114., 108.],
                vec![40., 58., 110., 102.],
                vec![16., 26., 46., 42.],
            ],
        );
        assert_eq!(m1 * m2, expected);
    }
    #[test]
    fn test_matrix_tuple_mul() {
        let m1 = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![1., 2., 3., 4.],
                vec![2., 4., 4., 2.],
                vec![8., 6., 4., 1.],
                vec![0., 0., 0., 1.],
            ],
        );
        let m2 = tuple::Tuple {
            x: 1.,
            y: 2.,
            z: 3.,
            w: 1.,
        };
        let expected = tuple::Tuple {
            x: 18.,
            y: 24.,
            z: 33.,
            w: 1.,
        };
        assert_eq!(m1 * m2, expected);
    }
    #[test]
    fn test_matrix_identity_mul() {
        let m1 = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![0., 1., 2., 4.],
                vec![1., 2., 4., 8.],
                vec![2., 4., 8., 16.],
                vec![4., 8., 16., 32.],
            ],
        );

        assert_eq!(m1.clone() * matrix::Matrix::identity(), m1);
        assert_eq!(matrix::Matrix::identity() * m1.clone(), m1);
    }
    #[test]
    fn test_matrix_transpose() {
        let m = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![0., 9., 3., 0.],
                vec![9., 8., 0., 8.],
                vec![1., 8., 5., 3.],
                vec![0., 0., 5., 8.],
            ],
        );
        let expected = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![0., 9., 1., 0.],
                vec![9., 8., 8., 0.],
                vec![3., 0., 5., 5.],
                vec![0., 8., 3., 8.],
            ],
        );

        assert_eq!(m.transpose(), expected);
    }
    #[test]
    fn test_matrix_determinant() {
        let m = matrix::Matrix::from(2, 2, vec![vec![1., 5.], vec![-3., 2.]]);
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn test_matrix_submatix_1() {
        let m = matrix::Matrix::from(
            3,
            3,
            vec![vec![1., 5., 0.], vec![-3., 2., 7.], vec![0., 6., -3.]],
        );
        let expected = matrix::Matrix::from(2, 2, vec![vec![-3., 2.], vec![0., 6.]]);
        assert_eq!(m.submatrix(0, 2), expected);
    }

    #[test]
    fn test_matrix_submatix_2() {
        let m = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![-6., 1., 1., 6.],
                vec![-8., 5., 8., 6.],
                vec![-1., 0., 8., 2.],
                vec![-7., 1., -1., 1.],
            ],
        );
        let expected = matrix::Matrix::from(
            3,
            3,
            vec![vec![-6., 1., 6.], vec![-8., 8., 6.], vec![-7., -1., 1.]],
        );
        assert_eq!(m.submatrix(2, 1), expected);
    }

    #[test]
    fn test_matrix_minor() {
        let m = matrix::Matrix::from(
            3,
            3,
            vec![vec![3., 5., 0.], vec![2., -1., -7.], vec![6., -1., 5.]],
        );
        assert_eq!(m.submatrix(1, 0).determinant(), 25.);
        assert_eq!(m.minor(1, 0), 25.);
    }

    #[test]
    fn test_matrix_cofactor() {
        let m = matrix::Matrix::from(
            3,
            3,
            vec![vec![3., 5., 0.], vec![2., -1., -7.], vec![6., -1., 5.]],
        );

        assert_eq!(m.minor(0, 0), -12.);
        assert_eq!(m.cofactor(0, 0), -12.);

        assert_eq!(m.minor(1, 0), 25.);
        assert_eq!(m.cofactor(1, 0), -25.);
    }

    #[test]
    fn test_matrix_3x3_determinant() {
        let m = matrix::Matrix::from(
            3,
            3,
            vec![vec![1., 2., 6.], vec![-5., 8., -4.], vec![2., 6., 4.]],
        );

        assert_eq!(m.cofactor(0, 0), 56.);
        assert_eq!(m.cofactor(0, 1), 12.);
        assert_eq!(m.cofactor(0, 2), -46.);

        assert_eq!(m.determinant(), -196.);
    }

    #[test]
    fn test_matrix_4x4_determinant() {
        let m = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![-2., -8., 3., 5.],
                vec![-3., 1., 7., 3.],
                vec![1., 2., -9., 6.],
                vec![-6., 7., 7., -9.],
            ],
        );

        assert_eq!(m.cofactor(0, 0), 690.);
        assert_eq!(m.cofactor(0, 1), 447.);
        assert_eq!(m.cofactor(0, 2), 210.);
        assert_eq!(m.cofactor(0, 3), 51.);
        assert_eq!(m.determinant(), -4071.);
    }

    #[test]
    fn test_matrix_is_inversible_1() {
        let m = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![6., 4., 4., 4.],
                vec![5., 5., 7., 6.],
                vec![4., -9., 3., -7.],
                vec![9., 1., 7., -6.],
            ],
        );

        assert_eq!(m.is_inversible(), true);
    }

    #[test]
    fn test_matrix_is_inversible_2() {
        let m = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![-4., 2., -2., -3.],
                vec![9., 6., 2., 6.],
                vec![0., -5., 1., -5.],
                vec![0., 0., 0., -0.],
            ],
        );

        assert_eq!(m.is_inversible(), false);
    }

    #[test]
    fn test_matrix_inverse() {
        let m = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![-5., 2., 6., -8.],
                vec![1., -5., 1., 8.],
                vec![7., 7., -6., -7.],
                vec![1., -3., 7., 4.],
            ],
        );
        let expected = matrix::Matrix::from(
            4,
            4,
            vec![
                vec![0.21805, 0.45113, 0.24060, -0.04511],
                vec![-0.80827, -1.45677, -0.44361, 0.52068],
                vec![-0.07895, -0.22368, -0.05263, 0.19737],
                vec![-0.52256, -0.81391, -0.30075, 0.30639],
            ],
        );
        let inv = m.inverse().expect("Inverse should be found");

        assert_eq!(m.determinant(), 532.);
        assert_eq!(m.cofactor(2, 3), -160.);
        assert_eq!(inv[(3, 2)], -160. / 532.);
        assert_eq!(m.cofactor(3, 2), 105.);
        assert_eq!(inv[(2, 3)], 105. / 532.);
        for r in 0..4 as usize {
            for c in 0..4 as usize {
                assert!(inv[(r, c)] - expected[(r, c)] < 0.0001)
            }
        }
    }

    #[test]
    fn test_matrix_point_translation() {
        let m = matrix::Matrix::translation(5., -3., 2.);
        let p = tuple::Tuple::point(-3., 4., 5.);

        let expect = tuple::Tuple::point(2., 1., 7.);
        assert_eq!(m * p, expect);
    }

    #[test]
    fn test_matrix_point_translation_inverse() {
        let m = matrix::Matrix::translation(5., -3., 2.);
        let p = tuple::Tuple::point(-3., 4., 5.);

        let expect = tuple::Tuple::point(-8., 7., 3.);
        assert_eq!((m.inverse()).expect("Should be inverse") * p, expect);
    }

    #[test]
    fn test_matrix_vector_translation() {
        let m = matrix::Matrix::translation(5., -3., 2.);
        let p = tuple::Tuple::vector(-3., 4., 5.);

        let expect = tuple::Tuple::vector(-3., 4., 5.);
        assert_eq!(m * p, expect);
    }

    #[test]
    fn test_matrix_point_scale() {
        let m = matrix::Matrix::scale(2., 3., 4.);
        let p = tuple::Tuple::point(-4., 6., 8.);

        let expect = tuple::Tuple::point(-8., 18., 32.);
        assert_eq!(m * p, expect);
    }
    #[test]
    fn test_matrix_vector_scale() {
        let m = matrix::Matrix::scale(2., 3., 4.);
        let p = tuple::Tuple::vector(-4., 6., 8.);

        let expect = tuple::Tuple::vector(-8., 18., 32.);
        assert_eq!(m * p, expect);
    }

    #[test]
    fn test_matrix_vector_inverse_scale() {
        let m = matrix::Matrix::scale(2., 3., 4.);
        let p = tuple::Tuple::vector(-4., 6., 8.);

        let expect = tuple::Tuple::vector(-2., 2., 2.);
        assert_eq!((m.inverse().expect("Should inverse")) * p, expect);
    }

    #[test]
    fn test_matrix_point_reflection() {
        let m = matrix::Matrix::scale(-1., 1., 1.);
        let p = tuple::Tuple::point(2., 3., 4.);

        let expect = tuple::Tuple::point(-2., 3., 4.);
        assert_eq!(m * p, expect);
    }

    #[test]
    fn test_matrix_rot_x() {
        let half_quarter = matrix::Matrix::rot_x(PI / 4.);
        let full_quarter = matrix::Matrix::rot_x(PI / 2.);
        let p = tuple::Tuple::point(0., 1., 0.);

        let expect_half_quarter = tuple::Tuple::point(0., f64::sqrt(2.) / 2., f64::sqrt(2.) / 2.);
        let expect_full_quarter = tuple::Tuple::point(0., 0., 1.);

        assert_eq!(half_quarter * p, expect_half_quarter);

        let result_full_quarter = full_quarter * p;
        assert!(&result_full_quarter.x - expect_full_quarter.x < f64::EPSILON);
        assert!(&result_full_quarter.y - expect_full_quarter.y < f64::EPSILON);
        assert!(&result_full_quarter.z - expect_full_quarter.z < f64::EPSILON);
    }
    #[test]
    fn test_matrix_rot_y() {
        let half_quarter = matrix::Matrix::rot_y(PI / 4.);
        let full_quarter = matrix::Matrix::rot_y(PI / 2.);
        let p = tuple::Tuple::point(0., 0., 1.);

        let expect_half_quarter = tuple::Tuple::point(f64::sqrt(2.) / 2., 0., f64::sqrt(2.) / 2.);
        let expect_full_quarter = tuple::Tuple::point(1., 0., 0.);

        assert_eq!(half_quarter * p, expect_half_quarter);

        let result_full_quarter = full_quarter * p;
        assert!(&result_full_quarter.x - expect_full_quarter.x < f64::EPSILON);
        assert!(&result_full_quarter.y - expect_full_quarter.y < f64::EPSILON);
        assert!(&result_full_quarter.z - expect_full_quarter.z < f64::EPSILON);
    }
    #[test]
    fn test_matrix_rot_z() {
        let half_quarter = matrix::Matrix::rot_z(PI / 4.);
        let full_quarter = matrix::Matrix::rot_z(PI / 2.);
        let p = tuple::Tuple::point(0., 1., 0.);

        let expect_half_quarter = tuple::Tuple::point(-f64::sqrt(2.) / 2., f64::sqrt(2.) / 2., 0.);
        let expect_full_quarter = tuple::Tuple::point(-1., 0., 0.);

        assert_eq!(half_quarter * p, expect_half_quarter);

        let result_full_quarter = full_quarter * p;
        assert!(&result_full_quarter.x - expect_full_quarter.x < f64::EPSILON);
        assert!(&result_full_quarter.y - expect_full_quarter.y < f64::EPSILON);
        assert!(&result_full_quarter.z - expect_full_quarter.z < f64::EPSILON);
    }
    #[test]
    fn test_shearing_1() {
        let m = matrix::Matrix::shearing(1.0, 0., 0., 0., 0., 0.);
        let p = tuple::Tuple::point(2., 3., 4.);
        let expect = tuple::Tuple::point(5., 3., 4.);
        assert_eq!(m * p, expect);
    }

    #[test]
    fn test_shearing_2() {
        let m = matrix::Matrix::shearing(0.0, 1., 0., 0., 0., 0.);
        let p = tuple::Tuple::point(2., 3., 4.);
        let expect = tuple::Tuple::point(6., 3., 4.);
        assert_eq!(m * p, expect);
    }
    #[test]
    fn test_shearing_3() {
        let m = matrix::Matrix::shearing(0.0, 0., 1., 0., 0., 0.);
        let p = tuple::Tuple::point(2., 3., 4.);
        let expect = tuple::Tuple::point(2., 5., 4.);
        assert_eq!(m * p, expect);
    }
    #[test]
    fn test_shearing_4() {
        let m = matrix::Matrix::shearing(0.0, 0., 0., 1., 0., 0.);
        let p = tuple::Tuple::point(2., 3., 4.);
        let expect = tuple::Tuple::point(2., 7., 4.);
        assert_eq!(m * p, expect);
    }
    #[test]
    fn test_shearing_5() {
        let m = matrix::Matrix::shearing(0.0, 0., 0., 1., 0., 0.);
        let p = tuple::Tuple::point(2., 3., 4.);
        let expect = tuple::Tuple::point(2., 7., 4.);
        assert_eq!(m * p, expect);
    }
    #[test]
    fn test_shearing_6() {
        let m = matrix::Matrix::shearing(0.0, 0., 0., 0., 1., 0.);
        let p = tuple::Tuple::point(2., 3., 4.);
        let expect = tuple::Tuple::point(2., 3., 6.);
        assert_eq!(m * p, expect);
    }
    #[test]
    fn test_shearing_7() {
        let m = matrix::Matrix::shearing(0.0, 0., 0., 0., 0., 1.);
        let p = tuple::Tuple::point(2., 3., 4.);
        let expect = tuple::Tuple::point(2., 3., 7.);
        assert_eq!(m * p, expect);
    }

    #[test]
    fn test_matrix_chain() {
        let p = tuple::Tuple::point(1., 0., 1.);
        let a = matrix::Matrix::rot_x(PI / 2.);
        let b = matrix::Matrix::scale(5., 5., 5.);
        let c = matrix::Matrix::translation(10., 5., 7.);

        let p2 = a * p;
        assert!(p2.x - 1.0 < f64::EPSILON);
        assert!(p2.y - (-1.) < f64::EPSILON);
        assert!(p2.z - 0. < f64::EPSILON);

        let p3 = b * p2;
        assert!(p3.x - 5.0 < f64::EPSILON);
        assert!(p3.y - (-5.) < f64::EPSILON);
        assert!(p3.z - 0. < 0.00000001);

        let p4 = c * p3;
        assert!(p4.x - 15.0 < f64::EPSILON);
        assert!(p4.y - (-0.) < f64::EPSILON);
        assert!(p4.z - 7. < f64::EPSILON);
    }
}
