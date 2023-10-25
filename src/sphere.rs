use crate::{aabb::Aabb, hittable::*, interval::Interval, material::Material, ray::*, vec3::*, rtweekend::PI};

#[derive(Clone)]
pub struct Sphere<M: Material> {
    pub center1: Vec3,
    pub radius: f64,
    pub mat: M,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl<M: Material> Sphere<M> {
    pub fn new(center1: Vec3, radius: f64, mat: M) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center1,
            radius,
            mat,
            is_moving: false,
            center_vec: Vec3::default(),
            bbox: Aabb::new_points(&(center1 - rvec), &(center1 + rvec)),
        }
    }

    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f64, mat: M) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_points(&(center1 - rvec), &(center1 + rvec));
        let box2 = Aabb::new_points(&(center2 - rvec), &(center2 + rvec));
        Self {
            center1,
            radius,
            mat,
            is_moving: true,
            center_vec: center2 - center1,
            bbox: Aabb::new_aabb(&box1, &box2),
        }
    }

    fn sphere_center(&self, time: f64) -> Point3 {
        self.center1 + time * self.center_vec
    }

    fn get_sphere_uv(&self, p: Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / phi;

        (u, v)
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
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
        let (u, v) = self.get_sphere_uv(outward_normal);

        Some(HitRecord {
            p,
            normal,
            t,
            front_face,
            u,
            v,
            mat: &self.mat,
        })
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
