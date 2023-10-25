use std::sync::Arc;

use crate::{aabb::Aabb, interval::*, material::Material, ray::*, vec3::Vec3};

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

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> &Aabb;
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
