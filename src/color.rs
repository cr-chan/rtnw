use crate::{interval::Interval, vec3::Vec3};
pub type Color = Vec3;

const INTENSITY: Interval = Interval {
    min: 0.0,
    max: 0.999,
};

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

impl Color {
    pub fn color(&self, sample_per_pixel: i32) -> (i32, i32, i32) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / sample_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        (
            (256.0 * INTENSITY.clamp(r)) as i32,
            (256.0 * INTENSITY.clamp(g)) as i32,
            (256.0 * INTENSITY.clamp(b)) as i32,
        )
    }
}
