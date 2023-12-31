use crate::{matrix::Matrix, tuple::Tuple};

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Object {
    Sphere(Matrix),
}
impl Eq for Object {}

pub fn transform(o: &Object, m: &Matrix) -> Object {
    match o {
        Object::Sphere(s) => Object::Sphere(s * m),
    }
}

pub fn normal_at(o: &Object, world_point:Tuple) ->  Tuple{
    let object_transform = match o {
        Object::Sphere(transform) => transform
    };
    let object_point = object_transform.inverse().expect("Should inverse")*world_point;
    let object_normal = object_point - Tuple::point(0.,0., 0.);
    let mut world_normal = object_transform.inverse().expect("Should inverse").transpose()*object_normal;
    world_normal.to_vector();
    return world_normal.normalize();
    
}

pub fn reflect(in_vector: &Tuple, normal:&Tuple) ->  Tuple{
    in_vector - (normal*2.)*in_vector.dot(normal)
}
#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{matrix::Matrix, object::transform, tuple::Tuple};

    use super::{Object, normal_at, reflect};

    #[test]
    fn default_transformation() {
        let a = Object::Sphere(Matrix::identity());
        match a {
            Object::Sphere(a) => assert_eq!(a, Matrix::identity()),

        }
    }

    #[test]
    fn default_translation() {
        let o = Object::Sphere(Matrix::identity());
        let m = Matrix::translation(2., 3., 4.);
        let o = transform(&o, &m);
        match o {
            Object::Sphere(a) => assert_eq!(a, m),

        }
    }

    #[test]
    fn test_normal_at_1() {
        let s = Object::Sphere(Matrix::identity());
        let n = normal_at(&s, Tuple::point(1., 0., 0.));
        assert_eq!(n,Tuple::vector(1., 0., 0.));
    }

    #[test]
    fn test_normal_at_2() {
        let s = Object::Sphere(Matrix::identity());
        let n = normal_at(&s, Tuple::point(0., 1., 0.));
        assert_eq!(n,Tuple::vector(0.,1., 0.));
    }

    #[test]
    fn test_normal_at_3() {
        let s = Object::Sphere(Matrix::identity());
        let n = normal_at(&s, Tuple::point(0., 0., 1.));
        assert_eq!(n,Tuple::vector(0., 0., 1.));
    }

    #[test]
    fn test_normal_at_4() {
        let s = Object::Sphere(Matrix::identity());
        let n = normal_at(&s, Tuple::point(f64::sqrt(3.)/3., f64::sqrt(3.)/3., f64::sqrt(3.)/3.));
        assert_eq!(n,Tuple::vector(f64::sqrt(3.)/3., f64::sqrt(3.)/3.,f64::sqrt(3.)/3.));
    }

    #[test]
    fn test_normal_on_a_translate_sphere() {
        let s = Object::Sphere(Matrix::identity());
        let s = transform(&s, &Matrix::translation(0., 1., 0.));
        let n = normal_at(&s, Tuple::point(0., 1.70711, -0.70711));
        let expected = Tuple::vector(0., 0.70711, -0.70711);
        assert!(n.x-expected.x<0.001);
        assert!(n.y-expected.y<0.001);
        assert!(n.z-expected.z<0.001);
        assert!(n.w-expected.w<0.001);
    }
    #[test]
    fn test_normal_on_a_scale_sphere() {
        let s = Object::Sphere(Matrix::identity());
        let m = Matrix::scale(1., 0.5, 1.)*Matrix::rot_z(PI/5.);
        let s = transform(&s, &m);
        let n = normal_at(&s, Tuple::point(0., f64::sqrt(2.)/2., -f64::sqrt(2.)/2.));
        let expected = Tuple::vector(0., 0.97014, -0.24254);
        assert!(n.x-expected.x<0.001);
        assert!(n.y-expected.y<0.001);
        assert!(n.z-expected.z<0.001);
        assert!(n.w-expected.w<0.001);
    }

    #[test]
    fn test_reflect_1() {
            let v = Tuple::vector(1., -1., 0.);
            let n = Tuple::vector(0.,1., 0.);
            let r = reflect(&v, &n);
            assert_eq!(r, Tuple::vector(1., 1., 0.));
    }
    #[test]
    fn test_reflect_2() {
            let v = Tuple::vector(0., -1., 0.);
            let n = Tuple::vector(f64::sqrt(2.)/2.,f64::sqrt(2.)/2., 0.);
            let r = reflect(&v, &n);
            let expected =  Tuple::vector(1., 0., 0.);
            assert!(r.x-expected.x<0.001);
            assert!(r.y-expected.y<0.001);
            assert!(r.z-expected.z<0.001);
            assert!(r.w-expected.w<0.001);
    }

}
