use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Point3,
    vec3::Vec3,
};

pub struct Quad<M: Material> {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: M,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl<M: Material> Quad<M> {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: M) -> Self {
        let n = Vec3::cross(u, v);
        let normal = Vec3::unit_vector(n);
        let d = Vec3::dot(normal, q);
        let w = n / Vec3::dot(n, n);
        Self {
            q,
            u,
            v,
            mat,
            bbox: set_bounding_box(q, u, v),
            normal,
            d,
            w,
        }
    }
}

impl<M: Material> Hittable for Quad<M> {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let denom = Vec3::dot(self.normal, r.direction());

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - Vec3::dot(self.normal, r.origin())) / denom;

        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.point_at_parameter(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(self.w, Vec3::cross(planar_hitpt_vector, self.v));
        let beta = Vec3::dot(self.w, Vec3::cross(self.u, planar_hitpt_vector));

        if let Some((u, v)) = is_interior(alpha, beta) {
            let (normal, front_face) = HitRecord::set_face_normal(r, self.normal);
            Some(HitRecord {
                p: intersection,
                normal,
                t,
                u,
                v,
                front_face,
                mat: &self.mat,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

fn set_bounding_box(q: Point3, u: Vec3, v: Vec3) -> Aabb {
    Aabb::new_points(&q, &(q + u + v)).pad()
}

fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
    if (a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b) {
        return None;
    }

    Some((a, b))
}
