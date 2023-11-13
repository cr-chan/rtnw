use image::io::Reader;

use crate::{
    color::Color, interval::Interval, perlin::Perlin, ray::Point3, vec3::Vec3,
};

pub trait Texture: Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

#[derive(Clone, Copy)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> Self {
        Self { color_value: c }
    }

    pub fn new_from_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        self.color_value
    }
}

#[derive(Clone, Copy)]
pub struct CheckerTexture<O: Texture, E: Texture> {
    odd: O,
    even: E,
    inv_scale: f64,
}

impl<O: Texture, E: Texture> CheckerTexture<O, E> {
    pub fn new(scale: f64, even: E, odd: O) -> Self {
        Self {
            odd,
            even,
            inv_scale: 1.0 / scale,
        }
    }
}

impl<O: Texture, E: Texture> Texture for CheckerTexture<O, E> {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor();
        let y_integer = (self.inv_scale * p.y()).floor();
        let z_integer = (self.inv_scale * p.z()).floor();

        let is_even = (x_integer + y_integer + z_integer) % 2.0 == 0.0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    image_width: usize,
    image_height: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let open = match Reader::open(filename) {
            Ok(image) => image,
            Err(_) => panic!("ERROR: Could not load image file \"{}\".", filename),
        };

        let decode = match open.decode() {
            Ok(image) => image.to_rgb8(),
            Err(_) => panic!("ERROR: Could not decode image file \"{}\".", filename),
        };

        let (width, height) = decode.dimensions();

        let data = decode.into_raw();

        Self {
            data,
            image_width: width as usize,
            image_height: height as usize,
            bytes_per_scanline: (width * 3) as usize,
        }
    }

    fn clamp(x: usize, low: usize, high: usize) -> usize {
        std::cmp::max(low, std::cmp::min(x, high - 1))
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        if self.image_height == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image_width as f64) as usize;
        let j = (v * self.image_height as f64) as usize;

        let pixel = if !self.data.is_empty() {
            let x = Self::clamp(i, 0, self.image_width);
            let y = Self::clamp(j, 0, self.image_height);
            let start = (y * self.bytes_per_scanline) + (x * 3);

            &self.data[start..start + 3]
        } else {
            &[255, 0, 255]
        };

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        let s = self.scale * p;
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (s.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

impl NoiseTexture {
    pub fn new(sc: f64) -> NoiseTexture {
        Self {
            noise: Perlin::default(),
            scale: sc,
        }
    }
}
