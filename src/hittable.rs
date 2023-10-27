use std::sync::Arc;

use crate::{
    aabb::Aabb,
    interval::*,
    material::Material,
    ray::*,
    rtweekend::{degrees_to_radians, INFINITY},
    vec3::Vec3,
};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: &'a dyn Material,
}

impl HitRecord<'_> {
    pub fn set_face_normal(ray: &Ray, outward_normal: Vec3) -> (Vec3, bool) {
        let front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (normal, front_face)
    }
}

pub trait Hittable: Sync + Send{
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &Aabb;
}

#[derive(Clone)]
pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            object: p.clone(),
            offset: displacement,
            bbox: *p.bounding_box() + displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let offset_r = Ray::new_time(r.origin() - self.offset, r.direction(), r.time());

        if let Some(mut rec) = self.object.hit(&offset_r, ray_t) {
            rec.p += self.offset;
            return Some(HitRecord {
                p: rec.p,
                normal: rec.normal,
                t: rec.t,
                u: rec.u,
                v: rec.v,
                front_face: rec.front_face,
                mat: rec.mat,
            });
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - j as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            object: p,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_points(&min, &max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotate_r = Ray::new_time(origin, direction, r.time());

        if let Some(mut rec) = self.object.hit(&rotate_r, ray_t) {
            rec.p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            rec.p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            rec.normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            rec.normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            return Some(HitRecord {
                p: rec.p,
                normal: rec.normal,
                t: rec.t,
                u: rec.u,
                v: rec.v,
                front_face: rec.front_face,
                mat: rec.mat,
            });
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

#[derive(Clone, Default)]
pub struct HittableList {
    pub object: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new(object: Arc<dyn Hittable>) -> Self {
        Self {
            object: vec![object],
            bbox: Aabb::default(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.bbox = Aabb::new_aabb(&self.bbox, object.bounding_box());
        self.object.push(Arc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = ray_t.max;

        for object in self.object.iter() {
            if let Some(hit) = object.hit(r, &mut Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
