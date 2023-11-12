use crate::{
    aabb::Aabb, hittable::*, interval::Interval, material::Material, ray::*, rtweekend::PI, vec3::*,
};

#[derive(Clone, Copy)]
pub struct Sphere<M: Material> {
    center1: Vec3,
    radius: f64,
    mat: M,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl<M: Material> Sphere<M> {
    pub fn new(center1: Vec3, center2: Vec3, radius: f64, mat: M, is_moving: bool) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        match is_moving {
            false => Sphere {
                center1,
                radius,
                mat,
                is_moving,
                center_vec: Vec3::default(),
                bbox: Aabb::new_from_points(center1 - rvec, center1 + rvec),
            },

            true => Sphere {
                center1,
                radius,
                mat,
                is_moving,
                center_vec: center2 - center1,
                bbox: {
                    let box1 = Aabb::new_from_points(center1 - rvec, center1 + rvec);
                    let box2 = Aabb::new_from_points(center2 - rvec, center2 + rvec);
                    Aabb::new_from_boxes(box1, box2)
                },
            },
        }
    }

    fn sphere_center(&self, time: f64) -> Vec3 {
        self.center1 + self.center_vec * time
    }

    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            self.sphere_center(r.time())
        } else {
            self.center1
        };
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = -(half_b + sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.point_at_parameter(t);
        let outward_normal = (p - center) / self.radius;
        let (normal, front_face) = HitRecord::set_face_normal(r, outward_normal);
        let (u, v) = Self::get_sphere_uv(outward_normal);

        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            mat: &self.mat,
            u,
            v,
        })
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox)
    }
}
