use crate::{color::Color, object::reflect, point_light::Point_Light, tuple::Tuple};
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambiant: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}
impl Material {
    pub fn new(color: Color, ambiant: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Material {
            color,
            ambiant,
            diffuse,
            specular,
            shininess,
        }
    }
    pub fn default() -> Self {
        Material {
            color: Color::new(1., 1., 1.),
            ambiant: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

pub fn lighting(
    material: &Material,
    light: &Point_Light,
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
) -> Color {
    let effective_color = material.color * light.intensity;
    let lightv = (light.position - *point).normalize();
    let ambient = effective_color * material.ambiant;
    let light_dot_normal = lightv.dot(&normalv);
    let mut diffuse = Color::new(0., 0., 0.);
    let mut specular = Color::new(0., 0., 0.);
    if light_dot_normal >= 0. {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = reflect(&-lightv, normalv);
        let reflect_fot_eye = reflectv.dot(eyev);
        if reflect_fot_eye >= 0. {
            let factor = f64::powf(reflect_fot_eye, material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    ambient + diffuse + specular
}
#[cfg(test)]
mod tests {

    use crate::{
        color::Color,
        point_light::{self, Point_Light},
        tuple::Tuple,
    };

    use super::lighting;
    use super::Material;
    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.color, Color::new(1., 1., 1.));
        assert_eq!(m.ambiant, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn test_light_shading_86() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Point_Light::new(Color::new(1., 1., 1.), Tuple::point(0., 0., -10.));

        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }
    #[test]
    fn test_light_shading_86_2() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., f64::sqrt(2.) / 2., -f64::sqrt(2.) / 2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Point_Light::new(Color::new(1., 1., 1.), Tuple::point(0., 0., -10.));

        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_light_shading_87() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Point_Light::new(Color::new(1., 1., 1.), Tuple::point(0., 10., -10.));

        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert!(result.r - 0.7364 < 0.0001);
        assert!(result.g - 0.7364 < 0.0001);
        assert!(result.b - 0.7364 < 0.0001);
    }
    #[test]
    fn test_light_shading_87_2() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., f64::sqrt(2.) / 2., -f64::sqrt(2.) / 2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Point_Light::new(Color::new(1., 1., 1.), Tuple::point(0., 10., -10.));

        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert!(result.r - 1.6364 < 0.0001);
        assert!(result.g - 1.6364 < 0.0001);
        assert!(result.b - 1.6364 < 0.0001);
    }
    #[test]
    fn test_light_shading_88() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = Point_Light::new(Color::new(1., 1., 1.), Tuple::point(0., 0., 10.));

        let result = lighting(&m, &light, &position, &eyev, &normalv);
        assert!(result.r - 0.1 < 0.0001);
        assert!(result.g - 0.1 < 0.0001);
        assert!(result.b - 0.1 < 0.0001);
    }
}
