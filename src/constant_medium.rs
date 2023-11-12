use std::f64::consts::E;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::*,
    material::Material,
    rtweekend::{self, random_double},
    vec3::Vec3,
};

pub struct ConstantMedium<M: Material> {
    boudary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: M,
}

impl<M: Material> ConstantMedium<M> {
    pub fn new(b: Box<dyn Hittable>, d: f64, mat: M) -> Self {
        Self {
            boudary: b,
            neg_inv_density: -1.0 / d,
            phase_function: mat,
        }
    }
}

#[allow(const_item_mutation)]

impl<M: Material> Hittable for ConstantMedium<M> {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval) -> Option<HitRecord> {
        const ENABLE_DEBUG: bool = false;
        let debugging: bool = ENABLE_DEBUG && random_double() < 0.00001;

        if let Some(mut rec1) = self.boudary.hit(r, &Interval::UNIVERSE) {
            if let Some(mut rec2) = self
                .boudary
                .hit(r, &Interval::new(rec1.t + 0.0001, rtweekend::INFINITY))
            {
                if debugging {
                    eprintln!("\nray_tmin={}, ray_tmax={}", rec1.t, rec2.t);
                }
                if rec1.t < ray_t.min {
                    rec1.t = ray_t.min;
                }

                if rec2.t > ray_t.max {
                    rec2.t = ray_t.max;
                }

                if rec1.t >= rec2.t {
                    return None;
                }

                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = r.direction().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_double().log(E);

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let p = r.point_at_parameter(t);

                Some(HitRecord {
                    p,
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    t,
                    u: 0.0,
                    v: 0.0,
                    front_face: true,
                    mat: &self.phase_function,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> std::option::Option<Aabb> {
        self.boudary.bounding_box()
    }
}
