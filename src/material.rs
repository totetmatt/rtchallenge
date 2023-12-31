
use crate::color::Color;
pub struct Material {
    color: Color,
    ambiant:f64,
    diffuse:f64,
    specular:f64,
    shininess:f64
}
impl Material {
    pub fn new(color:Color,ambiant:f64, diffuse:f64,specular:f64,shininess:f64) -> Self {
        Material {
            color,
            ambiant,
            diffuse,
            specular,
            shininess
        }
    }
    pub fn default() -> Self {
        Material {
            color:Color::new(1.,1.,1.),
            ambiant:0.1,
            diffuse:0.9,
            specular:0.9,
            shininess:200.0
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::{color::Color, tuple::Tuple};

    use super::Material;

   
    #[test]
    fn default_material() {
        let m = Material::default();

        assert_eq!(m.color,Color::new(1., 1., 1.));
        assert_eq!(m.ambiant,0.1);
        assert_eq!(m.diffuse,0.9);
        assert_eq!(m.shininess,200.0);
    }
}