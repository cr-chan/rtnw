use std::{ops::*, sync::Arc};

use crate::{
    interval::Interval,
    ray::{Point3, Ray},
    vec3::Vec3, rtweekend::INFINITY, hittable::Hittable,
};

#[derive(Clone, Copy, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
        Self {
            x: *x,
            y: *y,
            z: *z,
        }
    }

    pub fn new_points(a: &Point3, b: &Point3) -> Self {
        Self {
            x: Interval::new(a[0].min(b[0]), a[0].max(b[0])),
            y: Interval::new(a[1].min(b[1]), a[1].max(b[1])),
            z: Interval::new(a[2].min(b[2]), a[2].max(b[2])),
        }
    }

    pub fn new_aabb(box0: &Aabb, box1: &Aabb) -> Self {
        Self {
            x: Interval::new_interval(&box0.x, &box1.x),
            y: Interval::new_interval(&box0.y, &box1.y),
            z: Interval::new_interval(&box0.z, &box1.z),
        }
    }

    pub fn pad(&self) -> Self {
        let delta = 0.0001;
        let new_x = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };
        let new_y = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };
        let new_z = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };

        Aabb::new(&new_x, &new_y, &new_z)
    }

    pub fn axis(&self, n: usize) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn longest_axis(&self) -> usize {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();

        if x_size > y_size {
            if x_size > z_size {
                0
            } else {
                2
            }
        } else if y_size > z_size {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a).min - orig) * inv_d;
            let mut t1 = (self.axis(a).max - orig) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub const UNIVERSE: Aabb = Aabb {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    pub fn surrounding_box(objects: &[Arc<dyn Hittable>]) -> Self {
        let mut min_point = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max_point = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for object in objects {
            let bbox = object.bounding_box();
            min_point.e[0] = min_point.x().min(bbox.x.min);
            min_point.e[1] = min_point.y().min(bbox.y.min);
            min_point.e[2] = min_point.z().min(bbox.z.min);
            max_point.e[0] = max_point.x().max(bbox.x.max);
            max_point.e[1] = max_point.y().max(bbox.y.max);
            max_point.e[2] = max_point.z().max(bbox.z.max);
        }

        Self::new_points(&min_point, &max_point)
    }

    pub fn surface_area(&self) -> f64 {
        let dx = self.x.max - self.x.min;
        let dy = self.y.max - self.y.min;
        let dz = self.z.max - self.z.min;
        2.0 * (dx*dy + dx*dz + dy*dz)
    }
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
