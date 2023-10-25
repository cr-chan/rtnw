use crate::vec3::Vec3;

#[derive(Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

pub type Point3 = Vec3;

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn point_at_parameter(self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
