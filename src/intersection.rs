use crate::object::Object;
use std::cmp::Eq;
use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub o: Object,
}
impl Intersection {
    pub fn new(t: f64, o: Object) -> Self {
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

    use crate::{intersection::Intersection, object::Object};
    #[test]
    fn test_intersection() {
        let sphere = Object::sphere();
        let a = Intersection {
            t: 1.,
            o: sphere.clone(),
        };

        assert_eq!(a.t, 1.);

        assert_eq!(a.o, sphere);
    }
}
