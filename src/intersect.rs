use crate::intersection;
use crate::intersection::Intersection;
use crate::object;
use crate::object::Object;
use crate::ray;
use crate::tuple;
use std::collections::BinaryHeap;

type Intersections = BinaryHeap<Intersection>;
pub fn intersect(r: &ray::Ray, s: &object::Object) -> Intersections {
    let ray = match s {
        Object::Sphere(m) => r.transform(&m.inverse().expect("Should be inversable")),
    };
    let sphere_to_ray = ray.origin - tuple::Tuple::point(0., 0., 0.);
    let a = ray.direction.dot(&ray.direction);
    let b = 2. * ray.direction.dot(&sphere_to_ray);
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0. {
        BinaryHeap::new()
    } else {
        BinaryHeap::from([
            intersection::Intersection::new((-b - f64::sqrt(discriminant)) / (2.0 * a), s.clone()),
            intersection::Intersection::new((-b + f64::sqrt(discriminant)) / (2.0 * a), s.clone()),
        ])
    }
}

pub fn hit(i: &Intersections) -> Option<&Intersection> {
    BinaryHeap::from_iter(i.iter().filter(|x| x.t > 0.))
        .peek()
        .map(|x| *x)
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use crate::intersect::intersect;
    use crate::matrix::Matrix;
    use crate::{intersection, object, ray, tuple};

    use crate::intersect::hit;
    use crate::intersect::Intersections;
    #[test]
    fn test_intersect_2solutions() {
        let r = ray::Ray::new(
            tuple::Tuple::point(0., 0., -5.),
            tuple::Tuple::vector(0., 0., 1.),
        );
        let s = object::Object::Sphere(Matrix::identity());

        let mut intersections = intersect(&r, &s);
        assert_eq!(intersections.len(), 2);

        let a = intersections.pop().expect("Should have solutions");
        assert_eq!(a.t, 4.0);
        assert_eq!(a.o, object::Object::Sphere(Matrix::identity()));

        let b = intersections.pop().expect("Should have solutions");
        assert_eq!(b.t, 6.0);
        assert_eq!(b.o, object::Object::Sphere(Matrix::identity()));
    }
    #[test]
    fn test_intersect_1solution() {
        let r = ray::Ray::new(
            tuple::Tuple::point(0., 1., -5.),
            tuple::Tuple::vector(0., 0., 1.),
        );
        let s = object::Object::Sphere(Matrix::identity());

        let mut intersections = intersect(&r, &s);
        assert_eq!(intersections.len(), 2);

        let a = intersections.pop().expect("Should have solutions");
        assert_eq!(a.t, 5.0);
        assert_eq!(a.o, object::Object::Sphere(Matrix::identity()));

        let b = intersections.pop().expect("Should have Intersection");
        assert_eq!(b.t, 5.0);
        assert_eq!(b.o, object::Object::Sphere(Matrix::identity()));
    }
    #[test]
    fn test_intersect_miss() {
        let r = ray::Ray::new(
            tuple::Tuple::point(0., 2., 5.),
            tuple::Tuple::vector(0., 0., 1.),
        );
        let s = object::Object::Sphere(Matrix::identity());

        let intersections = intersect(&r, &s);
        assert_eq!(intersections.len(), 0)
    }

    #[test]
    fn test_intersect_inside() {
        let r = ray::Ray::new(
            tuple::Tuple::point(0., 0., 0.),
            tuple::Tuple::vector(0., 0., 1.),
        );
        let s = object::Object::Sphere(Matrix::identity());

        let mut intersections = intersect(&r, &s);
        assert_eq!(intersections.len(), 2);

        let a = intersections.pop().expect("Should have solutions");
        assert_eq!(a.t, -1.0);
        assert_eq!(a.o, object::Object::Sphere(Matrix::identity()));

        let b = intersections.pop().expect("Should have Intersection");
        assert_eq!(b.t, 1.0);
        assert_eq!(b.o, object::Object::Sphere(Matrix::identity()));
    }
    #[test]
    fn test_intersect_outside() {
        let r = ray::Ray::new(
            tuple::Tuple::point(0., 0., 5.),
            tuple::Tuple::vector(0., 0., 1.),
        );
        let s = object::Object::Sphere(Matrix::identity());

        let mut intersections = intersect(&r, &s);
        assert_eq!(intersections.len(), 2);

        let a = intersections.pop().expect("Should have solutions");
        assert_eq!(a.t, -6.0);
        assert_eq!(a.o, object::Object::Sphere(Matrix::identity()));

        let b = intersections.pop().expect("Should have Intersection");
        assert_eq!(b.t, -4.0);
        assert_eq!(b.o, object::Object::Sphere(Matrix::identity()));
    }
    #[test]
    fn test_hit_2_positive_intersection() {
        let o = object::Object::Sphere(Matrix::identity());
        let i1 = intersection::Intersection::new(1., o.clone());
        let i2 = intersection::Intersection::new(2., o.clone());
        let i: Intersections = BinaryHeap::from([i1, i2]);

        let h = hit(&i);

        assert_eq!(h.expect("Should not be None").t, 1.)
    }
    #[test]
    fn test_hit_1_positive_intersection() {
        let o = object::Object::Sphere(Matrix::identity());
        let i1 = intersection::Intersection::new(-1., o.clone());
        let i2 = intersection::Intersection::new(2., o.clone());
        let i: Intersections = BinaryHeap::from([i1, i2]);

        let h = hit(&i);

        assert_eq!(h.expect("Should not be None").t, 2.)
    }
    #[test]
    fn test_hit_no_positive_intersection() {
        let o = object::Object::Sphere(Matrix::identity());
        let i1 = intersection::Intersection::new(-1., o.clone());
        let i2 = intersection::Intersection::new(-2., o.clone());
        let i: Intersections = BinaryHeap::from([i1, i2]);

        let h = hit(&i);

        assert_eq!(h, None)
    }

    #[test]
    fn test_hit_multiple_intersection() {
        let o = object::Object::Sphere(Matrix::identity());
        let i1 = intersection::Intersection::new(5., o.clone());
        let i2 = intersection::Intersection::new(7., o.clone());
        let i3 = intersection::Intersection::new(-3., o.clone());
        let i4 = intersection::Intersection::new(2., o.clone());
        let i: Intersections = BinaryHeap::from([i1, i2, i3, i4]);

        let h = hit(&i);

        assert_eq!(h.expect("Should have solution").t, 2.)
    }
}
