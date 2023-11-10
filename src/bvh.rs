use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    ray::Ray,
};

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &mut HittableList) -> Self {
        let len = list.object.len();
        Self::build(&mut list.object, 0, len)
    }

    fn build(src_objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = Aabb::default();
        src_objects[start..end].iter().for_each(|obj| {
            bbox = Aabb::new_aabb(&bbox, obj.bounding_box());
        });
        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let objects = src_objects;

        let object_span = end - start;

        if object_span == 1 {
            Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox,
            }
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == std::cmp::Ordering::Less {
                Self {
                    left: objects[start].clone(),
                    right: objects[start + 1].clone(),
                    bbox,
                }
            } else {
                Self {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
                    bbox,
                }
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            let left = Arc::new(Self::build(objects, start, mid));
            let right = Arc::new(Self::build(objects, mid, end));
            Self { left, right, bbox }
        }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        a.bounding_box()
            .axis(axis_index)
            .min
            .partial_cmp(&b.bounding_box().axis(axis_index).min)
            .unwrap()
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let mut ray_t = *ray_t;
        if !self.bbox.hit(r, &mut ray_t) {
            return None;
        }

        let hit = self.left.hit(r, &mut ray_t);
        let mut ray = Interval::new(
            ray_t.min,
            if let Some(max) = hit {
                max.t
            } else {
                ray_t.max
            },
        );
        let result = self.right.hit(r, &mut ray).or(hit);

        result
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
