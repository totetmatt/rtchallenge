use std::ops;
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}
impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b,
        }
    }
}
impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Color {
        Color {
            r: self.r * _rhs,
            g: self.g * _rhs,
            b: self.b * _rhs,
        }
    }
}
impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color {
            r: self.r * _rhs.r,
            g: self.g * _rhs.g,
            b: self.b * _rhs.b,
        }
    }
}
impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, _rhs: f64) -> Color {
        Color {
            r: self.r / _rhs,
            g: self.g / _rhs,
            b: self.b / _rhs,
        }
    }
}
impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, _rhs: Color) -> Color {
        Color {
            r: self.r - _rhs.r,
            g: self.g - _rhs.g,
            b: self.b - _rhs.b,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::color;
    const EPSILON: color::Color = color::Color {
        r: f64::EPSILON,
        g: f64::EPSILON,
        b: f64::EPSILON,
    };
    #[test]
    fn is_a_point() {
        let a = color::Color::new(-0.5, 0.4, 1.7);
        assert_eq!(a.r, -0.5);
        assert_eq!(a.g, 0.4);
        assert_eq!(a.b, 1.7);
    }
    #[test]
    fn add_color() {
        let c1 = color::Color::new(0.9, 0.6, 0.75);
        let c2 = color::Color::new(0.7, 0.1, 0.25);
        let expect = color::Color::new(1.6, 0.7, 1.0);
        assert_eq!(c1 + c2, expect);
    }
    #[test]
    fn sub_color() {
        let c1 = color::Color::new(0.9, 0.6, 0.75);
        let c2 = color::Color::new(0.7, 0.1, 0.25);
        let expect = color::Color::new(0.2, 0.5, 0.5);
        assert!((c1 - c2) - expect < EPSILON);
    }
    #[test]
    fn mul_color_scala() {
        let c1 = color::Color::new(0.2, 0.3, 0.4);
        let expect = color::Color::new(0.4, 0.6, 0.8);
        assert!((c1 * 2.) - expect < EPSILON);
    }

    #[test]
    fn mul_color() {
        let c1 = color::Color::new(0.2, 0.3, 0.4);
        let c2 = color::Color::new(0.9, 1.0, 0.1);

        let expect = color::Color::new(0.9, 0.2, 0.04);
        assert!((c1 * c2) - expect < EPSILON);
    }
}
