use crate::object;
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub o: object::Object,
}
impl Intersection {
    pub fn new(t: f64, o: object::Object) -> Self {
        Intersection { t: t, o: o }
    }
}
impl Eq for Intersection {}
impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        self.t.partial_cmp(&other.t).map(|x| x.reverse())
    }
}
impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.total_cmp(&other.t).reverse()
    }
}
#[cfg(test)]
mod tests {

    use crate::{intersection::Intersection, matrix::Matrix, object};
    #[test]
    fn test_intersection() {
        let sphere = object::Object::Sphere(Matrix::identity());
        let a = Intersection { t: 1., o: sphere };

        assert_eq!(a.t, 1.);

        assert_eq!(a.o, object::Object::Sphere(Matrix::identity()));
    }
}
