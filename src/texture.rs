use crate::{color::Color, vec3::Vec3, rtimage::RtwImage, ray::Point3, interval::Interval, perlin::Perlin};

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
    image: RtwImage
}

impl ImageTexture {
    pub fn new(filname: &str) -> Self {
        Self {
            image: RtwImage::new(filname),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;
        let pixel = self.image.pixel_data(i, j);

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