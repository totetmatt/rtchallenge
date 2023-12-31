mod canvas;
mod color;
mod intersect;
mod intersection;
mod matrix;
mod object;
mod ray;
mod tuple;
mod point_light;
mod material;
use canvas::Canvas;
use color::Color;
use intersect::{hit, intersect};
use object::Object;
use tuple::Tuple;

use crate::matrix::Matrix;
use crate::ray::Ray;
fn main() {
    let canvas_pixels = 100;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let ray_origin = Tuple::point(0., 0., -5.);

    let wall_z = 10.0f64;
    let wall_size = 7.0f64;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let half = wall_size / 2.;
    let shape = Object::Sphere(Matrix::identity());
    let color = Color::new(1.0, 0., 0.);

    for y in 0..canvas_pixels {
        let world_y: f64 = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x: f64 = half - pixel_size * (x as f64);
            let position = Tuple::point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = intersect(&r, &shape);
            match hit(&xs) {
                Some(_) => canvas.set_pix((x, y), color),
                _ => (),
            }
        }
    }
    let mut file = std::fs::File::create("foo.ppm").expect("msg");
    canvas.to_ppm(&mut file);
}
