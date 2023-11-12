use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
            bbox: Aabb::default(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.bbox = Aabb::new_from_boxes(self.bbox, object.bounding_box().unwrap());
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox)
    }
}
