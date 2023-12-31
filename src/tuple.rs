use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 1f64,
        }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 0f64,
        }
    }
    pub fn is_point(&self) -> bool {
        self.w == 1f64
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0f64
    }
    pub fn to_vector(&mut self) {
        self.w = 0.;
    }
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }
    pub fn normalize(&self) -> Tuple {
        let mag: f64 = self.magnitude();
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}
impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, _rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}
impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, _rhs: f64) -> Tuple {
        Tuple {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
            w: self.w * _rhs,
        }
    }
}
impl ops::Mul<f64> for &Tuple {
    type Output = Tuple;

    fn mul(self, _rhs: f64) -> Tuple {
        Tuple {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
            w: self.w * _rhs,
        }
    }
}
impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, _rhs: f64) -> Tuple {
        Tuple {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
            w: self.w / _rhs,
        }
    }
}
impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, _rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}
impl ops::Sub<Tuple> for &Tuple {
    type Output = Tuple;

    fn sub(self, _rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}
impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple;
    #[test]
    fn is_a_point() {
        let a: tuple::Tuple = tuple::Tuple::point(4., -4., 3.);
        assert_eq!(a.x, 4.);
        assert_eq!(a.y, -4.);
        assert_eq!(a.z, 3.);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }
    #[test]
    fn is_a_vector() {
        let a = tuple::Tuple::vector(4., -4., 3.);
        assert_eq!(a.x, 4.);
        assert_eq!(a.y, -4.);
        assert_eq!(a.z, 3.);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }
    #[test]
    fn add_tuple() {
        let a1 = tuple::Tuple {
            x: 3.,
            y: -2.,
            z: 5.,
            w: 1.,
        };
        let a2 = tuple::Tuple {
            x: -2.,
            y: 3.,
            z: 1.,
            w: 0.,
        };
        let expected = tuple::Tuple {
            x: 1.,
            y: 1.,
            z: 6.,
            w: 1.,
        };
        let result = a1 + a2;
        assert_eq!(result, expected);
    }
    #[test]
    fn sub_points() {
        let p1 = tuple::Tuple::point(3., 2., 1.);
        let p2 = tuple::Tuple::point(5., 6., 7.);
        let expected = tuple::Tuple::vector(-2., -4., -6.);
        let result = p1 - p2;
        assert_eq!(result, expected);
        assert!(result.is_vector());
    }
    #[test]
    fn sub_point_and_vector() {
        let p1 = tuple::Tuple::point(3., 2., 1.);
        let p2 = tuple::Tuple::vector(5., 6., 7.);
        let expected = tuple::Tuple::point(-2., -4., -6.);
        let result = p1 - p2;
        assert_eq!(result, expected);
        assert!(result.is_point());
    }
    #[test]
    fn sub_vectors() {
        let p1 = tuple::Tuple::vector(3., 2., 1.);
        let p2 = tuple::Tuple::vector(5., 6., 7.);
        let expected = tuple::Tuple::vector(-2., -4., -6.);
        let result = p1 - p2;
        assert_eq!(result, expected);
        assert!(result.is_vector());
    }

    #[test]
    fn sub_vector_zero() {
        let zero = tuple::Tuple::vector(0., 0., 0.);
        let v = tuple::Tuple::vector(1., -2., 3.);
        let expected = tuple::Tuple::vector(-1., 2., -3.);
        let result = zero - v;
        assert_eq!(result, expected);
    }
    #[test]
    fn negate_vector() {
        let t = tuple::Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };
        let expected = tuple::Tuple {
            x: -1.,
            y: 2.,
            z: -3.,
            w: 4.,
        };
        let t = -t;
        assert_eq!(t, expected);
    }
    #[test]
    fn mul_tuple_scalar() {
        let t = tuple::Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };
        let expect = tuple::Tuple {
            x: 3.5,
            y: -7.,
            z: 10.5,
            w: -14.,
        };
        assert_eq!(t * 3.5, expect);
    }
    #[test]
    fn mul_tuple_fractional() {
        let t = tuple::Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };
        let expected = tuple::Tuple {
            x: 0.5,
            y: -1.,
            z: 1.5,
            w: -2.,
        };
        assert_eq!(t * 0.5, expected);
    }
    #[test]
    fn div_tuple() {
        let t = tuple::Tuple {
            x: 1.,
            y: -2.,
            z: 3.,
            w: -4.,
        };
        let expected = tuple::Tuple {
            x: 0.5,
            y: -1.,
            z: 1.5,
            w: -2.,
        };

        assert_eq!(t / 2., expected)
    }
    #[test]
    fn magnitude_vector_1() {
        let t = tuple::Tuple::vector(1., 0., 0.);
        let expected = 1.;
        assert_eq!(t.magnitude(), expected);
    }

    #[test]
    fn magnitude_vector_2() {
        let t = tuple::Tuple::vector(0., 1., 0.);
        let expected = 1.;
        assert_eq!(t.magnitude(), expected);
    }

    #[test]
    fn magnitude_vector_3() {
        let t = tuple::Tuple::vector(0., 0., 1.);
        let expected = 1.;
        assert_eq!(t.magnitude(), expected);
    }

    #[test]
    fn magnitude_vector_4() {
        let t = tuple::Tuple::vector(1., 2., 3.);
        let expected = f64::sqrt(14.);
        assert_eq!(t.magnitude(), expected);
    }

    #[test]
    fn magnitude_vector_5() {
        let t = tuple::Tuple::vector(-1., -2., -3.);
        let expected = f64::sqrt(14.);
        assert_eq!(t.magnitude(), expected);
    }

    #[test]
    fn normalize_vector_1() {
        let v = tuple::Tuple::vector(4., 0., 0.);
        let expected = tuple::Tuple::vector(1., 0., 0.);
        assert_eq!(v.normalize(), expected);
    }

    #[test]
    fn magnitude_normalize_vector() {
        let v = tuple::Tuple::vector(1., 2., 3.);
        let expected = 1.;
        assert_eq!(v.normalize().magnitude(), expected);
    }
    #[test]
    fn dot_vector() {
        let v1 = tuple::Tuple::vector(1., 2., 3.);
        let v2 = tuple::Tuple::vector(2., 3., 4.);
        let expect = 20.;
        assert_eq!(v1.dot(&v2), expect);
    }
    #[test]
    fn cross_vector() {
        let v1 = tuple::Tuple::vector(1., 2., 3.);
        let v2 = tuple::Tuple::vector(2., 3., 4.);

        let expected_v1v2 = tuple::Tuple::vector(-1., 2., -1.);
        let expected_v2v1 = tuple::Tuple::vector(1., -2., 1.);

        assert_eq!(v1.cross(&v2), expected_v1v2);

        assert_eq!(v2.cross(&v1), expected_v2v1);
    }
}
