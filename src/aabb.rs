use std::ops::Add;

use crate::{interval::Interval, vec3::Vec3};

#[derive(Default, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let _self = Self { x, y, z };
        _self.pad_to_minimums()
    }

    pub fn new_from_points(p0: Vec3, p1: Vec3) -> Self {
        let x = Interval::new(p0.x(), p1.x());
        let y = Interval::new(p0.y(), p1.y());
        let z = Interval::new(p0.z(), p1.z());
        let _self = Self { x, y, z };
        _self.pad_to_minimums()
    }

    pub fn new_from_boxes(box1: Aabb, box2: Aabb) -> Self {
        let x = Interval::new_from_interval(box1.x, box2.x);
        let y = Interval::new_from_interval(box1.y, box2.y);
        let z = Interval::new_from_interval(box1.z, box2.z);
        Self { x, y, z }
    }

    pub fn axis(&self, a: usize) -> Interval {
        match a {
            0 => self.x,
            1 => self.y,
            _ => self.z,
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, r: &crate::ray::Ray, t: &Interval) -> bool {
        let mut tmin = t.min;
        let mut tmax = t.max;

        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.axis(a).min - r.origin()[a]) * inv_d;
            let mut t1 = (self.axis(a).max - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };

            if tmax <= tmin {
                return false;
            }
        }
        true
    }

    fn pad_to_minimums(&self) -> Self {
        let delta = 0.0001;
        let x = if self.x.size() < delta {
            self.x.expand(delta)
        } else {
            self.x
        };
        let y = if self.y.size() < delta {
            self.y.expand(delta)
        } else {
            self.y
        };
        let z = if self.z.size() < delta {
            self.z.expand(delta)
        } else {
            self.z
        };
        Self { x, y, z }
    }

    pub const EMPTY: Aabb = Aabb {
        x: Interval {
            min: std::f64::INFINITY,
            max: std::f64::NEG_INFINITY,
        },
        y: Interval {
            min: std::f64::INFINITY,
            max: std::f64::NEG_INFINITY,
        },
        z: Interval {
            min: std::f64::INFINITY,
            max: std::f64::NEG_INFINITY,
        },
    };
}

impl Add<Vec3> for Aabb {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, rhs: Aabb) -> Self::Output {
        Aabb {
            x: self.x() + rhs.x,
            y: self.y() + rhs.y,
            z: self.z() + rhs.z,
        }
    }
}
