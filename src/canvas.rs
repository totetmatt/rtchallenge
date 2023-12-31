use super::color;
use std::io::Write;
pub struct Canvas {
    width: usize,
    height: usize,
    c: Vec<Vec<color::Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            c: vec![vec![color::Color::new(0., 0., 0.); width]; height],
        }
    }
    pub fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    pub fn set_pix(&mut self, coord: (usize, usize), c: color::Color) {
        self.c[coord.1][coord.0] = c;
    }
    pub fn get_pix(&self, coord: (usize, usize)) -> color::Color {
        self.c[coord.1][coord.0]
    }
    pub fn to_ppm<T: Write>(&self, writeable: &mut T) {
        let _ = writeable.write("P3\n".as_bytes());
        let _ = writeable.write(format!("{} {}\n", self.width, self.height).as_bytes());
        let _ = writeable.write("255\n".as_bytes());
        for row in self.c.iter() {
            let row_str = row
                .iter()
                .flat_map(|pix| {
                    let r: u8 = f64::min(255.0, f64::round(pix.r * 255.0)) as u8;
                    let g: u8 = f64::min(255.0, f64::round(pix.g * 255.0)) as u8;
                    let b: u8 = f64::min(255.0, f64::round(pix.b * 255.0)) as u8;

                    let r = format!("{r}");
                    let g = format!("{g}");
                    let b = format!("{b}");
                    vec![r, g, b]
                })
                .reduce(|mut acc, i| {
                    let l = acc.len();
                    if l % 70 <= (l + 1 + i.len()) % 70 {
                        acc.push(' ');
                        acc = acc + &i;
                    } else {
                        acc.push('\n');
                        acc = acc + &i;
                    }
                    acc
                });
            if row_str.is_some() {
                let s = row_str.expect("Should have lines");
                let _ = writeable.write(s.as_bytes());
            }

            let _ = writeable.write("\n".as_bytes());
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{canvas, color};
    #[test]
    fn test_canvas_init() {
        let canvas = canvas::Canvas::new(10, 20);
        assert_eq!(canvas.height, canvas.c.len());
        assert_eq!(canvas.width, canvas.c[0].len());
        let expect = color::Color::new(0., 0., 0.);
        for row in canvas.c.iter() {
            for cell in row.iter() {
                assert_eq!(*cell, expect);
            }
        }
    }
    #[test]
    fn test_canvas_set_get_pix() {
        let mut canvas = canvas::Canvas::new(10, 20);
        let expected = color::Color::new(1., 0., 0.);
        canvas.set_pix((2, 3), expected);
        let c = canvas.get_pix((2, 3));
        assert_eq!(c, expected);
        assert_eq!(canvas.get_pix((0, 0)), color::Color::new(0., 0., 0.));
    }

    #[test]
    fn test_canvas_write_ppm_header() {
        let canvas = canvas::Canvas::new(10, 20);
        let expected = r#"P3
10 20
255"#;

        let mut buf = std::io::BufWriter::new(Vec::new());

        canvas.to_ppm(&mut buf);

        let bytes = buf.into_inner().expect("Should be able into inner");
        let string_result = String::from_utf8(bytes).expect("Should to UTF 8");
        let collection: Vec<&str> = string_result.split("\n").collect();
        assert_eq!(collection[0..3].join("\n"), expected);
    }
    #[test]
    fn test_canvas_write_ppm_1() {
        let mut canvas = canvas::Canvas::new(5, 3);
        let expected = r#"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"#;
        let mut buf = std::io::BufWriter::new(Vec::new());
        canvas.set_pix(
            (0, 0),
            color::Color {
                r: 1.5,
                g: 0.,
                b: 0.,
            },
        );
        canvas.set_pix(
            (2, 1),
            color::Color {
                r: 0.,
                g: 0.5,
                b: 0.,
            },
        );
        canvas.set_pix(
            (4, 2),
            color::Color {
                r: -0.5,
                g: 0.,
                b: 1.,
            },
        );
        canvas.to_ppm(&mut buf);

        let bytes = buf.into_inner().expect("Should be able into inner");
        let string_result = String::from_utf8(bytes).expect("Should to UTF 8");

        let collection: Vec<&str> = string_result.split("\n").collect();
        assert_eq!(collection[3..6].join("\n"), expected);
    }
    #[test]
    fn test_canvas_write_ppm_2() {
        let mut canvas = canvas::Canvas::new(10, 2);
        let expected = r#"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153"#;
        let mut buf = std::io::BufWriter::new(Vec::new());
        for row in 0..2 {
            for col in 0..10 {
                canvas.set_pix(
                    (col, row),
                    color::Color {
                        r: 1.,
                        g: 0.8,
                        b: 0.6,
                    },
                );
            }
        }
        canvas.to_ppm(&mut buf);
        let bytes = buf.into_inner().expect("Should be able into inner");
        let string_result = String::from_utf8(bytes).expect("Should to UTF 8");
        let collection: Vec<&str> = string_result.split("\n").collect();
        assert_eq!(collection[3..7].join("\n"), expected);
    }
    #[test]
    fn test_canvas_write_ppm_last_char() {
        let canvas = canvas::Canvas::new(5, 3);
        let expected = '\n';
        let mut buf = std::io::BufWriter::new(Vec::new());
        canvas.to_ppm(&mut buf);
        let bytes = buf.into_inner().expect("Should be able into inner");
        let string_result = String::from_utf8(bytes).expect("Should to UTF 8");
        assert_eq!(string_result.chars().last(), Some(expected));
    }
}
