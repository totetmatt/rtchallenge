use crate::color::Color;
use crate::tuple::Tuple;
pub struct Point_Light {
    pub intensity: Color,
    pub position: Tuple,
}
impl Point_Light {
    pub fn new(intensity: Color, position: Tuple) -> Self {
        Point_Light {
            intensity,
            position,
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::{color::Color, tuple::Tuple};

    use super::Point_Light;

    #[test]
    fn default_point_light() {
        let i = Color::new(1., 1., 1.);
        let p = Tuple::point(0., 0., 0.);

        let pl = Point_Light::new(i, p);
        assert_eq!(pl.intensity, Color::new(1., 1., 1.));
        assert_eq!(pl.position, Tuple::point(0., 0., 0.))
    }
}
