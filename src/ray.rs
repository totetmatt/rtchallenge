use crate::matrix::Matrix;
use crate::tuple;
pub struct Ray {
    pub origin: tuple::Tuple,
    pub direction: tuple::Tuple,
}
impl Ray {
    pub fn new(origin: tuple::Tuple, direction: tuple::Tuple) -> Self {
        Ray { origin, direction }
    }
    pub fn position_at(&self, time: f64) -> tuple::Tuple {
        self.origin + (self.direction * time)
    }

    pub fn transform(&self, m: &Matrix) -> Self {
        Ray {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::intersect::intersect;
    use crate::matrix::Matrix;
    use crate::object::{transform, Object};
    use crate::tuple::Tuple;
    use crate::{ray, tuple};
    #[test]
    fn test_ray() {
        let origin = tuple::Tuple::point(1., 2., 3.);
        let direction = tuple::Tuple::vector(4., 5., 6.);
        let r = ray::Ray::new(origin, direction);

        assert_eq!(r.origin, tuple::Tuple::point(1., 2., 3.));
        assert_eq!(r.direction, tuple::Tuple::vector(4., 5., 6.));
    }
    #[test]
    fn test_position_at() {
        let r = ray::Ray::new(
            tuple::Tuple::point(2., 3., 4.),
            tuple::Tuple::vector(1., 0., 0.),
        );

        assert_eq!(r.position_at(0.), tuple::Tuple::point(2., 3., 4.));
        assert_eq!(r.position_at(1.), tuple::Tuple::point(3., 3., 4.));
        assert_eq!(r.position_at(-1.), tuple::Tuple::point(1., 3., 4.));
        assert_eq!(r.position_at(2.5), tuple::Tuple::point(4.5, 3., 4.));
    }

    #[test]
    fn test_ray_translate() {
        let r = ray::Ray::new(
            tuple::Tuple::point(1., 2., 3.),
            tuple::Tuple::vector(0., 1., 0.),
        );
        let m = Matrix::translation(3., 4., 5.);
        let result = r.transform(&m);
        assert_eq!(result.origin, tuple::Tuple::point(4., 6., 8.));
        assert_eq!(result.direction, tuple::Tuple::vector(0., 1., 0.))
    }

    #[test]
    fn test_ray_scale() {
        let r = ray::Ray::new(
            tuple::Tuple::point(1., 2., 3.),
            tuple::Tuple::vector(0., 1., 0.),
        );
        let m = Matrix::scale(2., 3., 4.);
        let result = r.transform(&m);
        assert_eq!(result.origin, tuple::Tuple::point(2., 6., 12.));
        assert_eq!(result.direction, tuple::Tuple::vector(0., 3., 0.))
    }

    #[test]
    fn test_ray_intersect_scale() {
        let r = ray::Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Object::sphere();
        let s = transform(&s, &Matrix::scale(2., 2., 2.));
        let mut xs = intersect(&r, &s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs.pop().expect("").t, 3.);
        assert_eq!(xs.pop().expect("").t, 7.);
    }
    #[test]
    fn test_ray_intersect_translate() {
        let r = ray::Ray::new(Tuple::point(0., 0., -5.), Tuple::vector(0., 0., 1.));
        let s = Object::sphere();
        let s = transform(&s, &Matrix::translation(5., 0., 0.));
        let xs = intersect(&r, &s);
        assert_eq!(xs.len(), 0);
    }
}
