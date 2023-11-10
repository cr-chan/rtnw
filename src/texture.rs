use std::sync::Arc;

use crate::{color::Color, perlin::Perlin, ray::Point3};

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}


pub struct SoildColor {
    color_value: Color,
}

impl SoildColor {
    pub fn new(c: Color) -> Self {
        Self { color_value: c }
    }

    pub fn new_color(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
}

impl Texture for SoildColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}


pub struct CheckerTexture {
    in_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            in_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_color(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            in_scale: 1.0 / scale,
            even: Arc::new(SoildColor::new(c1)),
            odd: Arc::new(SoildColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_integer = (self.in_scale * p.x()).floor() as i32;
        let y_integer = (self.in_scale * p.y()).floor() as i32;
        let z_integer = (self.in_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
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
