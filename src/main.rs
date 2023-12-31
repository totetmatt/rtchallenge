mod canvas;
mod color;
mod intersect;
mod intersection;
mod material;
mod matrix;
mod object;
mod point_light;
mod ray;
mod tuple;
use canvas::Canvas;
use color::Color;
use intersect::{hit, intersect};
use material::Material;
use object::{normal_at, Object};
use point_light::Point_Light;
use tuple::Tuple;

use crate::matrix::Matrix;
use crate::ray::Ray;
fn main() {
    let canvas_pixels = 1024;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let ray_origin = Tuple::point(0., 0., -5.);

    let wall_z = 10.0f64;
    let wall_size = 7.0f64;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let half = wall_size / 2.;
    let mut mat = Material::default();
    mat.color = Color::new(1., 0.2, 1.);
    let shape = Object::sphere_with_mat(mat);

    let light = Point_Light::new(Color::new(1., 1., 1.), Tuple::point(-10., 10., -10.));
    for y in 0..canvas_pixels {
        let world_y: f64 = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x: f64 = half - pixel_size * (x as f64);
            let position = Tuple::point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = intersect(&r, &shape);
            match hit(&xs) {
                Some(hit_point) => {
                    let point = r.position_at(hit_point.t);
                    let normal = normal_at(&hit_point.o, point);
                    let eye = -r.direction;
                    let color = match &hit_point.o {
                        Object::Sphere(_, mat) => {
                            material::lighting(&mat, &light, &point, &eye, &normal)
                        }
                    };

                    canvas.set_pix((x, y), color)
                }
                _ => (),
            }
        }
    }
    let mut file = std::fs::File::create("foo.ppm").expect("msg");
    canvas.to_ppm(&mut file);
}
